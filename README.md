# Rust Quiz Web Application

> A Rust full-stack quiz web application.

## Getting Set Up Locally

Using Docker Compose is the easiest way to run this locally,
unless you have SurrealDB installed locally and would like to configure it.

The SurrealDB instance creates a bind-mount in `/var/lib/surrealquizdata/`.
It was difficult allow SurrealDB to create a file database due to container permission issue.
We can circumvent the issue by giving the container a directory it has [free rein](https://www.vocabulary.com/articles/pardon-the-expression/free-rein-vs-free-reign) over.

```bash
sudo mkdir /var/lib/surrealquizdata
sudo chmod 777 /var/lib/surrealquizdata
```

Once you have that folder, assuming you have docker installed,
on my WSL2 Ubuntu instance, run the following commands in the root of this project:

```bash
docker compose build
docker compose up
```

The Leptos frontend _should_ be available on port 8080,
The Actix-Web backend _should_ be available on port 8000,
and the SurrealDB instance _should_ be accessible on port 8001.

## Development Cycle

Just setup a new branch called "develop".
The workflow will follow something like [Gitflow | Atlassian.com](https://www.atlassian.com/continuous-delivery/continuous-integration/trunk-based-development).
This project is (as of May 2024) still in early stages of development.
As such, there will be the "main" branch, which _should_ always be stable.
Then there is the "develop" branch, where features branches are merged into.
Only after proving changes to "develop" are stable should it be merged to "main".

When the project is deployed to the cloud, this will hopefully ensure stable deployments.

## Database

This project uses [SurrealDB](https://surrealdb.com/).
It has a very useful [surrealdb crate | crates.io](https://crates.io/crates/surrealdb)
that makes integration with Rust very simple.
The crate has [surrealdb docs | docs.rs](https://docs.rs/surrealdb/latest/surrealdb/index.html) documentation.

The database is being used in a docker container for development because I just typically do not install databases directly onto my machine.
SurrealDB can be accessed from the command line and data reviewed with SQL like commands.
Once you have your docker container database running, use the following command:

```bash
docker exec -i -t <container-name / id> /surreal sql -u user -p password --ns surreal --db quiz_app --pretty
```

This runs the `sql` command in the `/surreal` directory in the container.
We pass in the credentials, the namespace, database name, and request pretty formatting.
Adding notes so I do not forget command.

## Logging

Actix-Web does not simply log requests like some other frameworks.
A simple logger might due for some applications,
but with multiple threads serving many users,
an error could become hard to trace.
This is why logs should be easy to correlate.
The book "Zero to Production in Rust" by Luca Palmieri covers Telemetry in Chapter 4.
This project will follow the book's more complicated approach to logging.

## Application Configuration

> Probably going to have 2 configuration types.

### Local

### Production

## Used Crates

### FrontEnd

To format the component code in the `view!{ ... }` macros,

```bash
cargo install leptosfmt
```

- `cargo add leptos@0.6 --features=csr`
  - Required for using Leptos as our frontend framework.
- `cargo add leptos_router --features=csr`
  - Since this is SPA, we want to give illusion of routing with a router.
- `cargo add serde@1.0 --features=derive`
  - Required for serializing and deserialization
- `cargo add serde_json@1.0`
  - Required for serializing and deserialization
- `cargo add wasm-bindgen@0.2`
  - This is crate of WASM bindings to JavaScript web APIs, needed to fetch.
- `cargo add wasm-bindgen-futures@0.4`
  - This is crate of WASM bindings to JavaScript web APIs, needed Promises and futures (for fetching).

### BackEnd

- `cargo add actix-web@4.6`
  - Required for using Actix-Web as the backend framework.
- `cargo add tokio@1.37 --features=macros,rt-multi-thread`
  - Following Zero to Production, Tokio is an asynchronous runtime for Rust.
- `cargo add actix-cors@0.7`
  - Cannot get separate frontend without it, see [Cors docs](https://docs.rs/actix-cors/latest/actix_cors/)
- `cargo add tracing@0.1 --features=log`
  - Better logs for asynchronous applications
- `cargo add tracing-subscriber@0.3 --features=registry,env-filter`
  - To help implement and use the `tracing::Subscriber` trait
- `cargo add tracing-bunyan-formatter@0.3`
  - Handy crate to format logs as JSON
- `cargo add tracing-log@0.2`
  - When actix-web fires a log event, this crate can redirect logs to our tracing subscriber.
- `cargo add tracing-actix-web@0.7`
  - Provides `TracingLogger` to be used as middleware to collect telemetry data.
- `cargo add thiserror@1`
  - Provides procedural macro to derive `std::error::Error` trait.
- `cargo add anyhow@1`
  - Helps to simplify error handling by being like a catch-all error trait object.
- `cargo add --no-default-features reqwest --features=json,rustls-tls,cookies`
  - This is easy-to-use HTTP client, which is very helpful in testing
- `cargo add --dev wiremock`
  - Can spawn mock servers for testing.
  - This will only be used for testing purposes, no need to spawn mock servers in live code.
  - Actix-Web testing does not appear to necessarily spawn a server for integration tests.
- `cargo add serde@1 --features=derive`
  - Rust standard for data serialization and deserialization.
- `cargo add serde-aux@4.5`
  - Houses helpful function for casting types during deserialization.
- `cargo add serde_json`
  - Providing serialization and deserialization implementation for JSON format.
- `cargo add config`
  - Allows you to read and merge configuration from multiple sources.
