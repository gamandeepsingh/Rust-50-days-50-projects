# Day 10 — Authentication

A small Rust web API built with `axum` that demonstrates JWT-based authentication with user registration and login.

## What it does

- User registration with password hashing
- User login that returns a JWT token
- Protected routes that require a valid Bearer token
- JWT validation middleware

## Endpoints

- `POST /register` - Register a new user with JSON body `{ "username": "user", "password": "pass" }`
- `POST /login` - Login with credentials to receive a JWT token
- `GET /protected` - Access protected route (requires `Authorization: Bearer <token>` header)

## Run

```bash
cargo run
```

Server runs on `http://127.0.0.1:3000`

## Example usage

```bash
# Register a user
curl -X POST http://127.0.0.1:3000/register \
  -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"secret"}'

# Login and get token
curl -X POST http://127.0.0.1:3000/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"secret"}'

# Access protected route with token
curl -H "Authorization: Bearer <token>" \
  http://127.0.0.1:3000/protected
```

## Notes

- Passwords are hashed using bcrypt
- JWT tokens are created with a secret key
- Protected routes use middleware to validate tokens
- Unauthorized requests return a 401 status code
