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
