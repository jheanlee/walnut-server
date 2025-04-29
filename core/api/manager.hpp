//
// Created by Jhean Lee on 2025/4/29.
//

#ifndef BRANCH_VAULT_MANAGER_HPP
  #define BRANCH_VAULT_MANAGER_HPP
  #include <atomic>
  #include <mutex>

  #include <sys/un.h>
  #include "../message.hpp"

  void api_control_thread_func();
  void api_session_thread_func(int api_fd, sockaddr_un api_addr);
  void api_heartbeat_thread_func(std::atomic<bool> &flag_kill, std::atomic<bool> &flag_heartbeat, int &api_fd, std::mutex &send_mutex);


#endif //BRANCH_VAULT_MANAGER_HPP
