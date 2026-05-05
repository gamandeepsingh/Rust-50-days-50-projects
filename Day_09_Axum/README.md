# Day 09 — Axum

A small Rust web API built with `axum` and `tokio` that stores todos in memory and exposes basic CRUD endpoints.

## What it does

- Starts an HTTP server on `http://127.0.0.1:3000`
- Keeps todo items in an in-memory `HashMap`
- Supports creating, listing, updating, and deleting todos
- Logs each incoming request with a middleware layer

## Routes

- `GET /todos` - List all todos
- `POST /todos` - Create a new todo with a JSON body like `{ "title": "Learn Axum" }`
- `GET /todos/:id` - Fetch a single todo by id
- `PUT /todos/:id` - Mark a todo as completed
- `DELETE /todos/:id` - Delete a todo

## Run

```bash
cargo run
```

The server prints the local URL after startup.

## Example requests

```bash
curl http://127.0.0.1:3000/todos
curl -X POST http://127.0.0.1:3000/todos \
  -H 'Content-Type: application/json' \
  -d '{"title":"Learn Axum"}'
```

## Notes

- Data is not persisted; restarting the server clears all todos
- `PUT /todos/:id` sets `completed` to `true`
- The middleware prints each request method and path to the console
