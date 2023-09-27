server:
    cargo watch -q -c -w src/ -w .cargo/ -x "run"

dev:
    cargo watch -q -c -w examples/ -w .cargo/ -x "run --example quick_dev"

gen_key:
    cargo run --example gen_key

docker:
    docker run -d --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=postgres  postgres

psql:
    docker exec -it -u postgres pg psql

test:
    cargo watch -q -c -x "test -- --nocapture"