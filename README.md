# LLM Network

A peer-to-peer network application that allows users to communicate with an AI language model through a web interface. The application automatically discovers other instances on the local network and establishes peer-to-peer connections.

## Features

- Modern web interface for chatting with AI
- Automatic peer discovery on local networks
- Peer-to-peer networking capabilities
- Integration with local Ollama LLM
- Real-time chat interface with typing indicators

## Prerequisites

- Rust (latest stable version)
- Node.js (v16 or higher)
- Ollama with qwen2.5-coder:7b model installed

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/Omkar2k5/LLM-Network.git
   cd LLM-Network
   ```

2. Build the frontend:
   ```bash
   cd webpage
   npm install
   npm run build
   cd ..
   ```

3. Build the Rust application:
   ```bash
   cargo build --release
   ```

## Usage

1. Make sure Ollama is running with the qwen2.5-coder:7b model installed.

2. Run the application:
   ```bash
   ./target/release/instance
   ```

3. The application will automatically open your default browser to `http://localhost:8080/app/`

## Architecture

- **Backend**: Rust with Actix-web framework
- **Frontend**: React, TypeScript, and Tailwind CSS
- **LLM Integration**: Local Ollama instance
- **Network Discovery**: UDP broadcasts for peer discovery
- **Peer Communication**: TCP connections between instances

## Contributing

Feel free to submit issues and enhancement requests!

## Author

- Omkar2k5

## License

This project is licensed under the MIT License - see the LICENSE file for details. 