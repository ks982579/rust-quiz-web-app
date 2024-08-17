# Rust Quiz Web Application

> A Rust full-stack quiz web application.

## Application Configuration

There are currently two configuration types which are very similar, "local" and "production".
It's assumed most users would run the application in "local" mode, on their machine and access through 127.0.0.1.
Below are instructions to run the application through Docker.

### Local

The SSL certificates and NGINX configuration point to 'quiztestapp.io'.
As such, it's recommended the user update their local DNS to point their browser at 127.0.0.1.
On linux the file to update is `/etc/hosts`.
I run WSL through windows, so on my machine I must update the Windows DNS file
found in `C:\\Windows\Systems32\drivers\etc\hosts`.
In either case, add the following to the end of the file so the application can be found locally:

```
127.0.0.1 quiztestapp.io
127.0.0.1 www.quiztestapp.io
```

Then, in the root directory the user must build the application with the docker compose "build" command.
Then, they can run the docker compose "up" command (the `-d` flag is optional).
A list of commands are presented below:

```bash
docker compose -f compose-local.yaml build
docker compose -f compose-local.yaml up -d
docker compose -f compose-local.yaml down
docker compose -f compose-local.yaml logs
docker compose -f compose-local.yaml start
docker compose -f compose-local.yaml stop
docker compose -f compose-local.yaml exec <service_name> <command>
```

Exit the application with `CTRL+C` if not in detached terminal mode.
If the user passed in the `-d` flag, use the `down` or `stop` command.
When images are built, the user can enter an image's container for debugging with the following command:

```bash
docker run -it --rm <image_name> /bin/sh
```

I had permission issues initially with SurrealDB.
The SurrealDB instance creates a bind-mount in `/var/lib/surrealquizdata/`.
It was difficult allow SurrealDB to create a file database due to container permission issue.
We can circumvent the issue by giving the container a directory it has [free rein](https://www.vocabulary.com/articles/pardon-the-expression/free-rein-vs-free-reign) over.

```bash
sudo mkdir /var/lib/surrealquizdata
sudo chmod 777 /var/lib/surrealquizdata
```

### Production

This application is set for domain name "kevsquizapp.com".
Since I own that domain name, any other user launching a production version of this application
must update the "/frontend/nginx-prod.conf" file to point at their domain.
This also requires setting up DNS records on your hosting platform.

Additionally, the user must now set environment variables in their host machine.
See the `compose-prod.yaml` file to see which variables to set.
The "environment" attributes require preset variables.

Also, update the email address in the compose file specified

Set up the compose-prod.yaml file, which means our commands look like:

```bash
docker compose -f compose-prod.yaml up -d
docker compose -f compose-prod.yaml build
docker compose -f compose-prod.yaml down
docker compose -f compose-prod.yaml logs
docker compose -f compose-prod.yaml start
docker compose -f compose-prod.yaml stop
docker compose -f compose-prod.yaml exec <service_name> <command>
```

Or, in a bash profile like file, we can specify `export COMPOSE_FILE=compose-prod.yaml`.
I only have one configuration so need to uncomment code for production in nginx.conf.

### Development

For development, I do not use Docker for the application.
I do use Docker, after creating the application locally, to run the SurrealDB database.
However, I run the front-end with `trunk serve --port 8080`;
and the back-end with `cargo watch -x run`.
Both commands work well and provide hot reloading.

The Leptos frontend _should_ be available on port 8080,
The Actix-Web backend _should_ be available on port 8002,
and the SurrealDB instance _should_ be accessible on port 8000.

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
