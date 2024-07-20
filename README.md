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

## Testing

### Integration Testing

If you are running all back-end tests together, they must be run sequentially.
I could spin up a new database for each test case, but that also sounds like a lot of work.
There's also a `serial_test` crate to provide a `#[serial]` macro you stick under the `#[test]` macro
for tests you want to run sequentially.
Or, you can implement a mutex yourself with something like

```rust
use std::sync::Mutex;
use lazy_static::laxy_static;

lazy_static! {
  static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn example_test() {
  let _test_guard = TEST_MUTEX.lock().unwrap();
  // Add test logic below
}
```

Again, that's a bit of work.

When you run tests, use the following:

```bash
cargo test -- --test-threads=1
```

If tests are failing and you see an error there are "too many open files",
especially on Linux:

```bash
ulimit -n 10000
```

### End to End Testing

Working with Python and [RobotFrameWork](https://robotframework.org) as I am familiar with this tool.
Instead of working with Selenium I am choosing to work with [Playwright](https://playwright.dev) because I have read good things about this tool.
The [Browser Library](https://robotframework-browser.org) library is powered by Playwright.
So far, I have merely created a virtual environment in Python, my machine has version 3.10.
The documentation also state a requirement for Nodejs.
I have Node Version Manager (nvm) and just installed the latest long term support (lts) v20.15.1.

Now run:

```
python -m venv .venv
source ./.venv/bin/activate
pip install robotframework
pip install robotframework-browser
rfbrowser init
```

That should install the packages / libraries and tools required to run the end to end tests.
It is customary to setup a `run.py` file that sets up requirements for testing.
However, I will try to keep this very simple to start.
Run simple end-to-end tests with:

```
robot ./e2e/testsuites
```

It's actually recommended to be in the "e2e" folder because Robot will create a results directory where you run the tests,
unless specified to store them elsewhere.
The console will print results and there's a wonderful report generated as well.
RobotFrameWork is a batteries included test framework.

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
