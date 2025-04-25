# Supply Chain Backend API

A Rust-based RESTful API for managing supply chain operations.

## Features

- Product management
- Supplier management
- Order processing
- Inventory tracking

## Technologies

- Rust
- Actix Web (HTTP server framework)
- SQLx (SQL toolkit)
- PostgreSQL (database)

## Project Structure

```
backend/
├── src/
│   ├── api/           # API routes and handlers
│   ├── config/        # Configuration settings
│   ├── models/        # Data models
│   ├── services/      # Business logic
│   ├── utils/         # Utility functions
│   └── main.rs        # Application entry point
├── .env               # Environment variables
├── Cargo.toml         # Rust package configuration
└── README.md          # Documentation
```

## Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+

### Setup

1. **Clone the repository**

2. **Setup the database**

   Create a PostgreSQL database for the application.

3. **Configure environment variables**

   Copy the .env.example file to .env and update the values accordingly:

   ```
   DATABASE_URL=postgres://username:password@localhost/supplychain
   SERVER_PORT=8080
   RUST_LOG=info
   ```

4. **Install dependencies and run**

   ```bash
   # Install dependencies and build
   cargo build
   
   # Run migrations (when implemented)
   # cargo run --bin migrate
   
   # Start the server
   cargo run
   ```

5. **Access the API**

   The API will be available at `http://localhost:8080/api`

## API Endpoints

### Products

- `GET /api/products` - List all products
- `GET /api/products/{id}` - Get a specific product
- `POST /api/products` - Create a new product
- `PUT /api/products/{id}` - Update a product
- `DELETE /api/products/{id}` - Delete a product

### Suppliers

- `GET /api/suppliers` - List all suppliers
- `GET /api/suppliers/{id}` - Get a specific supplier
- `POST /api/suppliers` - Create a new supplier
- `PUT /api/suppliers/{id}` - Update a supplier
- `DELETE /api/suppliers/{id}` - Delete a supplier

### Orders

- `GET /api/orders` - List all orders
- `GET /api/orders/{id}` - Get a specific order
- `POST /api/orders` - Create a new order
- `PATCH /api/orders/{id}/status` - Update order status