//
// Created by Jhean Lee on 2025/4/15.
//

#ifndef BRANCH_VAULT_CONSOLE_HPP
  #define BRANCH_VAULT_CONSOLE_HPP

  #include <string>

  enum Level : int {
    CRITICAL = 50,
    ERROR = 40,
    WARNING = 30,
    INFO = 20,
    DEBUG = 10,
  };

  enum Code {
    SODIUM_INIT_FAILED,
    SQLITE_OPEN_FAILED,
    DEBUG_MSG,
  };

  void console(Level level, Code code, const char *detail, const std::string &function);

#endif //BRANCH_VAULT_CONSOLE_HPP
