//
// Created by Jhean Lee on 2025/4/29.
//
#include <cstring>
#include <exception>
#include <sys/socket.h>

#include "message.hpp"
#include "console.hpp"
#include "shared.hpp"

void Message::clear() {
  type = 0x00;
  string = "";
}

void Message::load(char *buffer) {
  if (strlen(buffer) == 0) throw std::length_error("empty message");
  if (strlen(buffer) > MESSAGE_MAX_STRING_SIZE + 1) throw std::length_error("message length exceeding limit");

  type = buffer[0];
  string = std::string(buffer + 1);
}

void Message::load(char *buffer, size_t buf_size) {
  if (strlen(buffer) == 0) throw std::length_error("empty message");
  if (strlen(buffer) > buf_size + 1) throw std::length_error("message length exceeding limit");

  type = buffer[0];
  string = std::string(buffer + 1);
}

void Message::dump(char *buffer) const {
  if (type <= 0x00) throw std::invalid_argument("type not specified");
  if (string.size() > MESSAGE_MAX_STRING_SIZE) throw std::length_error("message length exceeding limit");

  buffer[0] = type;
  strcat(buffer, string.c_str());
}

void Message::dump(char *buffer, size_t buf_size) const {
  if (type == '\0' || type < 0) throw std::invalid_argument("type not specified");
  if (string.size() > buf_size - 2) throw std::length_error("message length exceeding limit");

  buffer[0] = type;
  strcat(buffer, string.c_str());
}

int send_message(int &fd, char *buffer, size_t buffer_size, Message &message, std::mutex &send_mutex) {
  std::lock_guard<std::mutex> lock(send_mutex);

  try {
    std::memset(buffer, '\0', buffer_size);
    message.dump(buffer);
  } catch (const std::exception &err) {
    console(WARNING, MESSAGE_DUMP_FAILED, nullptr, "message::message::dump");
    return -1;
  }

  return send(fd, buffer, buffer_size, 0);
}

int recv_message(int &fd, char *buffer, size_t buffer_size, Message &message) {
  std::memset(buffer, '\0', buffer_size);
  int nbytes = recv(fd, buffer, buffer_size, 0);
  if (nbytes <= 0) return nbytes;

  try {
    message.load(buffer);
  } catch (const std::exception &err) {
    console(WARNING, MESSAGE_LOAD_FAILED, nullptr, "message::message::load");
    return -1;
  }

  return nbytes;
}

int read_message_non_block(int &fd, pollfd *pfds, char *buffer, size_t buffer_size, Message &message) {
  pfds[0] = {.fd = fd, .events = POLLIN | POLLPRI};
  int ready_for_call = poll(pfds, 1, API_POLL_TIMEOUT);
  if (ready_for_call < 0) {
    console(ERROR, API_SOCK_POLL_ERR, nullptr, "message::read_message_non_block");
    return -1;
  } else if (ready_for_call == 0) {
    return 0;
  } else {
    int recv_status = recv_message(fd, buffer, buffer_size, message);
    if (recv_status <= 0) return -1;
    return recv_status;
  }
}
