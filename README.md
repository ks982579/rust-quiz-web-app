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
