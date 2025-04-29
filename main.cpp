#include "core/encryption.hpp"
#include "core/shared.hpp"
#include "core/database/sqlite.hpp"

int main() {
  init_sodium();
  open_db(&shared_resources::db);



  return 0;
}
