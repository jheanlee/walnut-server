//
// Created by Jhean Lee on 2025/4/15.
//
#include <sstream>
#include <iostream>
#include <ctime>
#include <chrono>

#include "console.hpp"

#define RESET       "\033[0m"
#define RED         "\033[31m"
#define YELLOW      "\033[33m"
#define FAINT_GRAY  "\033[2;90m"
#define CYAN        "\033[36m"

void console(Level level, Code code, const char *detail, const std::string &function) {
  std::stringstream cout_buffer, msg_buffer;

  //  timestamp
  char strtime[32];
  time_t time = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());
  std::strftime(strtime, 32, "(%Y-%m-%d %H:%M:%S) ", std::gmtime(&time));
  cout_buffer << strtime;

  //  level
  switch (level) {
    case CRITICAL:
      cout_buffer << RED;
      cout_buffer << "[Critical] ";
      break;
    case ERROR:
      cout_buffer << RED;
      cout_buffer << "[Error] ";
      break;
    case WARNING:
      cout_buffer << YELLOW;
      cout_buffer << "[Warning] ";
      break;
    case INFO:
      cout_buffer << "[Info] ";
      break;
    case DEBUG:
      cout_buffer << "[DEBUG] ";
      break;
  }
  cout_buffer << RESET;

  switch (code) {
    case SODIUM_INIT_FAILED:
      msg_buffer << "Failed to initialise libsodium";
      break;
    case DEBUG_MSG:
      cout_buffer << CYAN << "DEBUG_MSG: " << RESET;
      break;
    case SQLITE_OPEN_FAILED:
      msg_buffer << "Failed to open database";
      break;
  }

  if (detail != nullptr) {
    msg_buffer << ' ';
    msg_buffer << detail;
  }

  cout_buffer << msg_buffer.str() << ' ';

  if (/*verbose_level <= DEBUG*/ true) {
    cout_buffer << FAINT_GRAY;
    cout_buffer << '(';
    cout_buffer << function;
    cout_buffer << ')';
    cout_buffer << RESET;
  }
  cout_buffer << '\n';

  std::cout << cout_buffer.str();
}