# rust_todo

Learning purposes only. The goal of this project is to put together a low-level app to learn more about async Rust and its ecosystem.

This app uses PostgreSQL via Docker container.

- Download PostgreSQL (which should include `psql`) and Docker for your environment
- Docker:

  - `docker pull postgres` to download the image
  - `docker run --name some-postgres -p 5432:5432 -e POSTGRES_PASSWORD=PASSWORD_OF_YOUR_CHOICE -d postgres` to run the container at port `5432`. Replace the `POSTGRES_PASSWORD` with the password of your choice.
  - `docker exec -it some-postgres bash` to login to the container and access the database via `psql`

- PostgreSQL:

```SQL
  CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 'f'
  );
```

- Env vars:

  - create a `.env` file at the root of this repo and enter `DATABASE_URL=postgres://postgres:PASSWORD_OF_YOUR_CHOICE@localhost/postgres`, replacing the placeholder with a password of your choice.

- Crates:
  - hyper
  - tokio
  - diesel
  - serde
  - futures
  - and other utils per `Cargo.toml`
