# EasyList Bot

EasyList is a private Telegram bot written in Rust that helps manage and organize TODO lists. You can simply add the bot to your chat, and every message will be formatted as a TODO list. Double tapping (liking) a message will mark it as done and add a ✅ emoji. Additionally, an Ansible playbook is included for deployment on a custom server.

## Features

- Supports Telegram group chats
- Uses Redis for storing and retrieving messages
- Automatically formats and deletes messages
- Updates messages based on reactions

## Installation

### Prerequisites

- Docker & Docker Compose
- Rust toolchain (for manual builds)
- Redis instance

### Using Docker Compose

1. Clone the repository:
   ```sh
   git clone https://github.com/aramd/easy_list.git
   cd easy_list
   ```

2. Build and start the services:
   ```sh
   docker-compose up --build -d
   ```

## Deployment

### Using Ansible on a Custom Server (e.g., Raspberry Pi)

Ansible playbook automates deployment to a Raspberry Pi:

```sh
ansible-playbook -i inventory.ini deploy.yml
```

## Configuration

Secrets are stored in `Secrets.toml`:

```toml
[tg]
token = "YOUR_TELEGRAM_BOT_TOKEN"

[redis]
uri = "redis://localhost:6379"
```

TODO:
- Provide a Secrets.toml.example file (consider using a .env file).
- Extract the Redis URI from Secrets.toml and create a separate config file.
- Make the Telegram thread ID (thread_id) configurable.
- Document the process of creating a bot with BotFather.
