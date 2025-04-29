//
// Created by Jhean Lee on 2025/4/15.
//

#include "shared.hpp"

namespace config {
  const char *db_path;
}

namespace shared_resources {
  sqlite3 *db;
  std::atomic<bool> flag_core_kill(false);
  std::atomic<bool> flag_api_kill(false);
}