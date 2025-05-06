#include <thread>

#include "core/encryption.hpp"
#include "core/shared.hpp"
#include "core/database/sqlite.hpp"
#include "core/api/manager.hpp"

int main() {
  init_sodium();
  open_db(&shared_resources::db);

  std::thread api_thread(api_control_thread_func);

  api_thread.join();

  return 0;
}
