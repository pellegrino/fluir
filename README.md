## Fluir

Blog engine written in Rust (Askama/Axum/TailwindCSS/htmx)


## Quickstart 🏁
(influenced by github.com/bitswired/rustgpt)

Jump right into it by following these steps:

1. Clone the repository.
2. Create a .env

```
MIGRATIONS_PATH=db/migrations
TEMPLATES_PATH=templates
DATABASE_URL=sqlite:db/db.db
DATABASE_PATH=db/db.db
OPENAI_API_KEY=<api-key> (only necessary for tests, users will add their own keys)
```

3. Install TailwindCSS Standalone in this repository: https://tailwindcss.com/blog/standalone-cli.
4. `cargo install just`: install Just
5. `just init`: install additional tools and migrate the db
6. `just dev`: concurrently run tailwind and cargo run in watch mode
