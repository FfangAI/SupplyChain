# Database Schema

## Suppliers

Stores information about product suppliers.

| Column        | Type         | Description                    |
|---------------|--------------|--------------------------------|
| id            | UUID         | Primary key                    |
| name          | VARCHAR(255) | Company name                   |
| contact_name  | VARCHAR(255) | Contact person name            |
| email         | VARCHAR(255) | Contact email                  |
| phone         | VARCHAR(50)  | Contact phone number           |
| address       | TEXT         | Street address                 |
| city          | VARCHAR(100) | City                           |
| state         | VARCHAR(100) | State/province                 |
| country       | VARCHAR(100) | Country                        |
| postal_code   | VARCHAR(20)  | Postal/zip code                |
| created_at    | TIMESTAMPTZ  | Record creation timestamp      |
| updated_at    | TIMESTAMPTZ  | Record last update timestamp   |

## Products

Stores information about products in inventory.

| Column         | Type          | Description                    |
|----------------|---------------|--------------------------------|
| id             | UUID          | Primary key                    |
| name           | VARCHAR(255)  | Product name                   |
| description    | TEXT          | Product description            |
| sku            | VARCHAR(100)  | Stock keeping unit (unique)    |
| price          | DECIMAL(10,2) | Unit price                     |
| stock_quantity | INTEGER       | Available quantity in stock    |
| supplier_id    | UUID          | Foreign key to suppliers       |
| created_at     | TIMESTAMPTZ   | Record creation timestamp      |
| updated_at     | TIMESTAMPTZ   | Record last update timestamp   |

## Orders

Stores customer orders.

| Column           | Type          | Description                  |
|------------------|---------------|------------------------------|
| id               | UUID          | Primary key                  |
| customer_name    | VARCHAR(255)  | Customer name                |
| customer_email   | VARCHAR(255)  | Customer email               |
| status           | VARCHAR(20)   | Order status                 |
| total_amount     | DECIMAL(10,2) | Total order amount           |
| shipping_address | TEXT          | Shipping address             |
| created_at       | TIMESTAMPTZ   | Record creation timestamp    |
| updated_at       | TIMESTAMPTZ   | Record last update timestamp |

## Order Items

Stores items within an order.

| Column     | Type          | Description                |
|------------|---------------|----------------------------|
| id         | UUID          | Primary key                |
| order_id   | UUID          | Foreign key to orders      |
| product_id | UUID          | Foreign key to products    |
| quantity   | INTEGER       | Quantity ordered           |
| unit_price | DECIMAL(10,2) | Price per unit at purchase |