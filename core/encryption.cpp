//
// Created by Jhean Lee on 2025/4/15.
//

#include <sodium.h>

#include "encryption.hpp"
#include "console.hpp"

void init_sodium() {
  if (sodium_init() == -1) {
    console(CRITICAL, SODIUM_INIT_FAILED, nullptr, "encryption::init_sodium");
    exit(EXIT_FAILURE);
  }
}
