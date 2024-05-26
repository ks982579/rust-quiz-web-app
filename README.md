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

## Logging

Actix-Web does not simply log requests like some other frameworks.
A simple logger might due for some applications,
but with multiple threads serving many users,
an error could become hard to trace.
This is why logs should be easy to correlate.
The book "Zero to Production in Rust" by Luca Palmieri covers Telemetry in Chapter 4.
This project will follow the book's more complicated approach to logging.

## Used Crates

### FrontEnd

- `cargo add leptos@0.6 --features=csr`
  - required for using Leptos as our frontend framework.

### BackEnd

- `cargo add actix-web@4.6`
  - Required for using Actix-Web as the backend framework.
- `cargo add tokio@1.37 --features=macros,rt-multi-thread`
  - Following Zero to Production, Tokio is an asynchronous runtime for Rust.
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
