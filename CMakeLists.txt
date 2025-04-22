cmake_minimum_required(VERSION 3.28)
project(Branch-Vault)
set(CMAKE_CXX_STANDARD 17)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")

add_executable(branch-vault main.cpp
        core/encryption.cpp
        core/encryption.hpp
        core/console.cpp
        core/console.hpp
        core/database/sqlite.cpp
        core/database/sqlite.hpp
        core/shared.cpp
        core/shared.hpp)

target_include_directories(branch-vault PRIVATE ${sodium_INCLUDE_DIR})

find_package(Sodium REQUIRED)
target_link_libraries(branch-vault PRIVATE ${sodium_LIBRARY_RELEASE})
find_package(SQLite3 REQUIRED)
target_link_libraries(branch-vault PRIVATE SQLite::SQLite3)
