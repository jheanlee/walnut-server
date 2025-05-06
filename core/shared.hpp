//
// Created by Jhean Lee on 2025/4/15.
//

#ifndef BRANCH_VAULT_SHARED_HPP
  #define BRANCH_VAULT_SHARED_HPP
  #include <string>
  #include <atomic>

  #include <sqlite3.h>

  #define SOCK_CONNECTION_LIMIT 5
  #define API_SOCK_PATH "/tmp/branch-vault.sock"
  #define API_POLL_TIMEOUT 100
  #define API_HEARTBEAT_TIMOUT 60000

  namespace config {
    extern const char *db_path;
  }

  namespace shared_resources {
    extern sqlite3 *db;
    extern std::atomic<bool> flag_core_kill;
    extern std::atomic<bool> flag_api_kill;
  }

#endif //BRANCH_VAULT_SHARED_HPP
