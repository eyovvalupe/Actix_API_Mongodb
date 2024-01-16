# Actix Web User Management API

## Introduction

This Actix web application serves as my first project in Rust using Actix web. It provides a simple REST API for managing user data and utilizes MongoDB as the backend database. The project aims to showcase the basic functionalities of Actix web, including handling HTTP requests, working with MongoDB, and implementing CRUD operations.

## Prerequisites

- Ensure that MongoDB is installed and running.
- Set the required environment variables (`HOST_URL` and `DATABASE_URL`) before running the application.

## Endpoints

1. **GET `/`**

   - **Description:** Displays a welcome message.
   - **Example:** `curl http://localhost:8080/`

2. **POST `/add_user`**

   - **Description:** Adds a new user to the database.
   - **Request Format:** JSON with `username` and `email` fields.
   - **Example:**
     ```bash
     curl -X POST -H "Content-Type: application/json" -d '{"username": "John", "email": "john@example.com"}' http://localhost:8080/add_user
     ```

3. **GET `/get_user/{email}`**

   - **Description:** Retrieves user details based on the provided email.
   - **Example:** `curl http://localhost:8080/get_user/john@example.com`

4. **PUT `/update_user/{email}`**

   - **Description:** Updates user information based on the provided email.
   - **Request Format:** JSON with `username` and `email` fields.
   - **Example:**
     ```bash
     curl -X PUT -H "Content-Type: application/json" -d '{"username": "John Doe", "email": "john@example.com"}' http://localhost:8080/update_user/john@example.com
     ```

5. **DELETE `/delete_user/{email}`**

   - **Description:** Deletes a user based on the provided email.
   - **Example:** `curl -X DELETE http://localhost:8080/delete_user/john@example.com`

## Note

- This project is my first exploration of Rust and Actix web.
- It demonstrates fundamental Actix web features and MongoDB integration.
- Ensure that the provided environment variables (`HOST_URL` and `DATABASE_URL`) are correctly set.
- Replace placeholders in the examples with actual data.
