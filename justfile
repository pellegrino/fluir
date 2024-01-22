set dotenv-load

init:
    cargo install cargo-watch
    cargo install diesel_cli --no-default-features --features postgres
    diesel setup
    just db-migrate

db-migrate:
    diesel migration run

dev-server:
	cargo watch -w src -w templates -w tailwind.config.js -w input.css -x run

dev-tailwind:
	./tailwindcss -i input.css -o assets/output.css --watch=always

dev:
	#!/bin/sh
	just dev-tailwind &
	pid1=$!
	just dev-server &
	pid2=$!
	trap "kill $pid1 $pid2" EXIT
	wait $pid1 $pid2
