<p align="center">
  <img src="dwarf-rs.png" alt="Logo" style="width:25%;"/>
</p>

# dwarf-rs

dwarf-rs is a lightweight and efficient open-source URL shortener written in Rust, leveraging the Actix-Web framework and PostgreSQL for persistence. Its primary focus is on simplicity, scalability, and performance, making it a great choice for personal or small-scale URL shortening services.

## Features

- Shorten long URLs with customizable slug size.
- Retrieve original URLs using their short slugs.
- Track visit counts for shortened URLs.
- High performance and scalability using Actix-Web.
- PostgreSQL for robust and reliable data storage.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Environment Variables](#environment-variables)
- [Usage](#usage)
  - [API Endpoints](#api-endpoints)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [TODO](#todo)
- [License](#license)

## Getting Started

### Prerequisites

To run dwarf-rs, you need:

- [Rust](https://www.rust-lang.org/) (1.82 or higher recommended)
- [PostgreSQL](https://www.postgresql.org/) (16 or higher recommended)
- [Docker](https://www.docker.com/) (optional, for containerized deployment)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/daemyn/dwarf-rs.git
   cd dwarf-rs
   ```

2. Create the `.env` file based on `.env.example`:

   ```bash
   cp .env.example .env
   ```

3. Configure your `.env` file with your database settings (see [Environment Variables](#environment-variables)).

4. Create and migrate your PostgreSQL database using `sqlx-cli`:

   ```bash
   sqlx database create
   sqlx migrate run
   ```

5. Build the project:

   ```bash
   cargo build
   ```

   Build in offline mode:

   ```bash
   SQLX_OFFLINE=true cargo build
   ```

6. Run the server:
   ```bash
   cargo run
   ```

### Environment Variables

The `.env` file must be created based on the provided `.env.example`. Configure it as follows:

```env
APP_PORT=3000
DATABASE_URL=postgresql://<username>:<password>@<host>/<database>
SLUG_SIZE=6
RUST_LOG=debug
```

| Variable       | Description                                     |
| -------------- | ----------------------------------------------- |
| `APP_PORT`     | Port on which the server will run.              |
| `DATABASE_URL` | Connection URL for PostgreSQL.                  |
| `SLUG_SIZE`    | Number of characters in the generated slugs.    |
| `RUST_LOG`     | Logging level (e.g., `debug`, `info`, `error`). |

## Usage

### API Endpoints

#### Create a Short URL

- **POST** `/api/v0/urls`

  **Request Body:**

  ```json
  {
    "target": "https://example.com"
  }
  ```

  **Response:**

  ```json
  {
    "id": 1,
    "slug": "abc123",
    "target": "https://example.com",
    "visit_count": 0,
    "created_at": "2025-01-21T00:00:00Z",
    "updated_at": "2025-01-21T00:00:00Z"
  }
  ```

#### Retrieve a URL by Slug

- **GET** `/api/v0/urls/{slug}`

  **Response:**

  ```json
  {
    "id": 1,
    "slug": "abc123",
    "target": "https://example.com",
    "visit_count": 5,
    "created_at": "2025-01-21T00:00:00Z",
    "updated_at": "2025-01-21T00:00:00Z"
  }
  ```

  #### Retrieve a URL by Slug

- **GET** `/{slug}`

  **Response:**
  Redirect to `target` url

- **GET** `/health`

  **Response:**
  - status code `200` when `ok`
  - status code `503` when `service unavailable`

## Project Structure

```plaintext
src/
├── errors.rs             # Custom error types
├── handlers.rs           # Route handlers for the application
├── main.rs               # Application entry point
├── models.rs             # Models for database and application state
├── services.rs           # Business logic for generating and retrieving URLs
├── utils.rs              # Utility functions (e.g., environment loading)
.env.example              # Example environment variables
```

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m 'Add feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
