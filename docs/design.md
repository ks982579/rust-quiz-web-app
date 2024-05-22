# Design

## High Level Directory Structure

Being a full-stack web application, having "frontend" and "backend" directories is common.
This project is a Cargo workspace.
However, the frontend and backend should be built in a way such that the
frontend can be swapped with another technology, like React or Svelte.
Cargo workspace makes builds for this particular stack easier.
