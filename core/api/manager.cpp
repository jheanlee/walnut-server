//
// Created by Jhean Lee on 2025/4/29.
//

#include <vector>
#include <thread>

#include <sys/socket.h>
#include <poll.h>
#include <unistd.h>

#include "manager.hpp"
#include "../shared.hpp"
#include "../console.hpp"
#include "../message.hpp"

void api_control_thread_func() {
  std::vector<std::thread> api_threads;

  int core_fd = socket(AF_UNIX, SOCK_STREAM, 0), api_fd, status; // unix domain socket, tcp

  sockaddr_un core_addr{.sun_family = AF_UNIX, .sun_path = API_SOCK_PATH}, api_addr{.sun_family = AF_UNIX};
  socklen_t core_addrlen = sizeof(core_addr);
  socklen_t api_addrlen = sizeof(struct sockaddr_un);

  struct pollfd pfds[1];

  unlink(API_SOCK_PATH);
  if (core_fd == -1) {
    console(ERROR, API_SOCK_CREATE_FAILED, nullptr, "api::api_control");
    shared_resources::flag_api_kill = true;
    return;
  }
  if (bind(core_fd, (struct sockaddr *) &core_addr, core_addrlen) == -1) {
    console(ERROR, API_SOCK_BIND_FAILED, nullptr, "api::api_control");
    shared_resources::flag_api_kill = true;
    return;
  }
  if (listen(core_fd, SOCK_CONNECTION_LIMIT) == -1) {
    console(ERROR, API_SOCK_LISTEN_FAILED, nullptr, "api::api_control");
    shared_resources::flag_api_kill = true;
    return;
  }
  console(INFO, API_LISTEN_STARTED, nullptr, "api::api_control");

  pfds[0] = {.fd = core_fd, .events = POLLIN | POLLPRI};
  while (!shared_resources::flag_api_kill) {
    status = poll(pfds, 1, API_POLL_TIMEOUT);
    if (status == 0) continue;
    else if (status < 0) {
      console(ERROR, API_SOCK_POLL_ERR, std::to_string(errno).c_str(), "api::api_control");
      continue;
    }

    api_fd = accept(core_fd, (struct sockaddr *) &api_addr, &api_addrlen);
    if (api_fd < 0) {
      console(ERROR, API_SOCK_ACCEPT_FAILED, nullptr, "api::api_control");
      continue;
    }

    api_threads.emplace_back(api_session_thread_func, api_fd, api_addr);
  }

  console(INFO, API_SERVICE_ENDED, nullptr, "api::api_control");
  shared_resources::flag_api_kill = true;

  for (std::thread &t : api_threads) {
    t.join();
  }
}

void api_session_thread_func(int api_fd, sockaddr_un api_addr) {
  console(INFO, API_CLIENT_CONNECTION_ACCEPTED, nullptr, "api::api_session");

  std::atomic<bool> flag_kill(false), flag_heartbeat(false);
  char inbuffer[256] = {0}, outbuffer[256] = {0};
  int recv_status;
  std::mutex send_mutex;
  std::thread heartbeat_thread(api_heartbeat_thread_func, std::ref(flag_kill), std::ref(flag_heartbeat), std::ref(api_fd), std::ref(send_mutex));

  Message message;
  message.clear();

  struct pollfd pfds[1];

  while (!shared_resources::flag_api_kill && !flag_kill) {
    recv_status = read_message_non_block(api_fd, pfds, inbuffer, sizeof(inbuffer), message);

    if (recv_status < 0) {
      flag_kill = true;
    } else if (recv_status > 0) {
      switch (message.type) {
        case API_HEARTBEAT:
          flag_heartbeat = true;
      }
    }
  }

  console(INFO, API_CLIENT_CONNECTION_ENDED, nullptr, "api::session");
  close(api_fd);
  flag_kill = true;
  heartbeat_thread.join();
}

void api_heartbeat_thread_func(std::atomic<bool> &flag_kill, std::atomic<bool> &flag_heartbeat, int &api_fd, std::mutex &send_mutex) {
  std::unique_lock<std::mutex> lock(send_mutex, std::defer_lock);

  char outbuffer[256] = {0};
  Message heartbeat_message = {.type = API_HEARTBEAT, .string = ""};

  std::chrono::system_clock::time_point timer;
  std::chrono::seconds heartbeat_duration;

  while (!flag_kill) {
    std::this_thread::sleep_for(std::chrono::milliseconds(API_HEARTBEAT_TIMOUT));

    //  send heartbeat message
    if (send_message(api_fd, outbuffer, sizeof(outbuffer), heartbeat_message, send_mutex) <= 0) {
      console(WARNING, MESSAGE_SEND_FAILED, nullptr, "api::api_heartbeat");
    }

    //  start timing
    timer = std::chrono::system_clock::now();
    flag_heartbeat = false;

    //  wait for heartbeat
    while (!flag_kill && !flag_heartbeat) {
      heartbeat_duration = std::chrono::duration_cast<std::chrono::seconds> (std::chrono::system_clock::now() - timer);

      if (heartbeat_duration > std::chrono::milliseconds(API_HEARTBEAT_TIMOUT)) {
        console(INFO, API_HEARTBEAT_TIMEOUT, nullptr, "api::api_heartbeat");
        flag_kill = true;
        return;
      }

      std::this_thread::sleep_for(std::chrono::milliseconds(1000));
    }
  }
}