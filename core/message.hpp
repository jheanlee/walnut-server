//
// Created by Jhean Lee on 2025/4/29.
//

#ifndef BRANCH_VAULT_MESSAGE_HPP
  #define BRANCH_VAULT_MESSAGE_HPP
  #include <string>
  #include <mutex>

  #include <poll.h>

  #define MESSAGE_MAX_STRING_SIZE 126
  #define MESSAGE_DEFAULT_BUFFER_SIZE 128

  class Message {
  public:
    char type;
    std::string string;

    void clear();
    void load(char *buffer);
    void load(char *buffer, size_t buf_size);
    void dump(char *buffer) const;
    void dump(char *buffer, size_t buf_size) const;
  };

  enum MessageType : char {
    //  0x00 special characters
    //  0x10 special characters

    //  0x20 reserved for future use

    //  0x30 api
    API_CONNECT =   0x30,
    API_EXIT =      0x31,
    API_HEARTBEAT = 0x32,
  };

  int send_message(int &fd, char *buffer, size_t buffer_size, Message &message, std::mutex &send_mutex);
  int recv_message(int &fd, char *buffer, size_t buffer_size, Message &message);
  int read_message_non_block(int &fd, pollfd *pfds, char *buffer, size_t buffer_size, Message &message);

#endif //BRANCH_VAULT_MESSAGE_HPP
