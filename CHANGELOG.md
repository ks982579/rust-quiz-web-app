# ChangeLog

> Hopefully, notable changes in this project will be documented here.
> Shout out to [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) for format.
> This project will also follow [Semantic Versioning](https://semver.org/) (as best I can).

## [Unreleased]

> Kind of a TODO section

### Adding

- v0.3 User Login Feature

### Changing

- UI design of user creation
- UI design for user dashboard
- On successful user creation, the user should be redirected to login page or Logged-in.

### Decprecating

No features on the block for deprecation.

### Removing

No features being removed.

### Fixing

No bugs currently logged.

### Security

No known security vulnerabilities reported as of yet.

## [0.3.0] - 2024-06-23

### Added

- v0.3 Back-end API to log users into application, issuing session cookies
- v0.3 Back-end middleware to check session cookies for protected endpoints
- v0.3 Created SurrealDB Session store for holding cookies
  - As a small application, reusing what we already have. If project grows, expand this to in memory database.
- v0.3 Added user dashboard to logged-in users in the UI.

### Changed

- User log-in page updated to include cookie acceptance check-box and send requests to backend
- Added several new crates again to implement new features, such as `actix-session`
- parts of front-end were refactored to reduce duplicated logic.

## [0.2.0] - 2024-06-12

### Added

- v0.2 Backend telemetry set up.
- v0.2 Adding SurrealDB instance to project.
- v0.2 Added CORs to project to allow communication from front-end to back-end.
- v0.2 Implemented back-end API to accept requests to create users.
- v0.2 Implemented password hashing with Argon2 so passwords are not stored as plain text.
- v0.2 Implemented front-end to communicate with back-end, sending requests to create users.

### Changed

- Added several new crates to implement new features.
