Rustlet
=======

**Rustlet** is a high-performance, production-ready HTTP server built in Rust. It leverages the asynchronous ecosystem provided by Tokio and the high-performance HTTP capabilities of Hyper, along with Tower for composable middleware. Designed with a modular architecture, Rustlet is ideal for developers seeking efficient and scalable web server solutions.

<img src="https://github.com/user-attachments/assets/13411764-136a-403d-bb59-5e03c658e1c0" width="500">




Features
--------

-   **Asynchronous I/O:** Powered by Tokio to handle thousands of concurrent connections.
-   **Efficient HTTP Handling:** Uses Hyper (v0.14) for robust HTTP parsing and connection management.
-   **Modular Architecture:** Organized into dedicated modules for configuration, routing, middleware, and server bootstrapping.
-   **Tower Middleware Integration:** Implements custom logging middleware using Tower's Service abstraction.
-   **Structured Logging:** Integrated with `tracing` for detailed request and response logging.

Architecture Overview
---------------------

-   **config:** Manages environment-based configuration (e.g., server address).
-   **handler:** Bridges incoming HTTP requests to the routing logic.
-   **router:** Maps incoming endpoints (like `/` and `/hello`) to appropriate responses.
-   **middleware:** Implements reusable middleware (e.g., logging of request and response data).
-   **server:** Sets up the TCP listener and bootstraps the HTTP server using Hyper's async connection handling.
-   **main:** Entry point that initializes logging, loads configuration, and launches the server.

Getting Started
---------------

### Prerequisites

-   Rust (version 1.65 or later)
-   Cargo package manager
-   Git (optional, for cloning the repository)

### Installation

Clone the repository:

```
git clone https://github.com/anubhav-auth/Rustlet.git
cd Rustlet

```

Install dependencies and build the project:

```
cargo build

```

### Running the Server

To run the server in development mode, execute:

```
cargo run

```

By default, the server listens on `127.0.0.1:3000`. You can override the default by setting the `SERVER_ADDR` environment variable:

```
export SERVER_ADDR="127.0.0.1:4000"
cargo run

```

<img src="https://github.com/user-attachments/assets/cfad037e-35d3-46fc-aa84-fc0dfce71c82" width="500">


### Testing Endpoints

Once the server is running:

-   Open your browser or use a tool like `curl` to test the endpoints:
    -   **Root Endpoint:**\
        Navigate to `http://127.0.0.1:3000/`\
        Expected response: **Welcome to Rustlet!**
    -   **Hello Endpoint:**\
        Navigate to `http://127.0.0.1:3000/hello`\
        Expected response: **Hello, world!**

  
<img src="https://github.com/user-attachments/assets/040c8bb7-73ce-4865-8350-ecfae20313bd" width="500">

### Example Using curl

```
curl http://127.0.0.1:3000/
curl http://127.0.0.1:3000/hello

```

Contributing
------------

Contributions are welcome! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Commit your changes with clear messages.
4.  Open a pull request for review.

For major changes, please open an issue first to discuss your proposed changes.

License
-------

This project is licensed under the MIT License. See the [LICENSE](https://github.com/anubhav-auth/Rustlet/blob/main/LICENSE) file for details.
