//
// Created by Jhean Lee on 2025/4/15.
//

#include "../shared.hpp"
#include "../console.hpp"

#include "sqlite.hpp"

//void open_db(sqlite3 **db) {
//  if (sqlite3_open(config::db_path, db) != SQLITE_OK) {
//    console(CRITICAL, SQLITE_OPEN_FAILED, sqlite3_errmsg(*db), "database::open_db");
//    exit(EXIT_FAILURE);
//  }
//}