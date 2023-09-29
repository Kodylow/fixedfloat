server:
    cargo watch -q -c -w src/ -w .cargo/ -x "run"

docker:
    docker-compose up -d

psql:
    docker exec -it -u postgres pg psql

test:
    cargo watch -q -c -x "test -- --nocapture"