# ChangeLog

> Hopefully, notable changes in this project will be documented here.
> Shout out to [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) for format.
> This project will also follow [Semantic Versioning](https://semver.org/) (as best I can).

## [Unreleased]

> Kind of a TODO section

### Adding

- v0.5 User ability to Edit Quizzes and Questions
- v0.5 User ability to take a quiz
- v0.6 Application updates to be suitable for hosting on cloud platform

### Changing

- v0.6 UI design of user creation
- v0.6 UI design for user dashboard
- v0.6 UI design for quiz creation
- v0.6 UI design for question creation

### Decprecating

No features on the block for deprecation.

### Removing

No features being removed.

### Fixing

No bugs currently logged.

### Security

No known security vulnerabilities reported as of yet.

## [0.5.0] - 2024-07-20

### Added

- v0.5 Back-end API to get quizzes owned by user.
- v0.5 Back-end API to get questions for a particular quiz, owned by the user.
- v0.5 Back-end API to delete quizzes.
- v0.5 Back-end API to delete questions.
- v0.5 Back-end API to edit quizzes.
- v0.5 Back-end API to edit questions.
- v0.5 Front-end UI to view existing quizzes owned by user.
- v0.5 Front-end UI to take a quiz that the user has made.
- v0.5 Front-end UI to delete a quiz owned by the user.
- v0.5 Front-end UI to delete a question owned by the user.
- v0.5 Front-end UI to edit quiz information owned by the user.
- v0.5 Front-end UI to edit a question to a quiz owned by the user.
- v0.5 Initialized the end-to-end Robot Framework test suite.

### Changed

- Refactored layout of components to improve the layout of the homepage.
- Updated CSS to improve design of existing UI.
- Updated the ID of quizzes from UUID to the default SurrealDB ID option.
  - This makes it easier to send information via query parameters.

### Removing

- Removed a Quizzes knowledge of child questions in database model.
  - It provided no benefit and complicated endpoints that updated questions.

## [0.4.0] - 2024-07-06

### Added

- v0.4 Back-end API to create quizzes
- v0.4 Back-end API to create questions for quizzes
- v0.4 Back-end API to handle user log out request
- v0.4 Front-end Dashboard has UI to create quizzes
- v0.4 Front-end Dashboard has UI to create questions for quizzes
- v0.4 Front-end Dashboard has button to log user out of application
- v0.4 Front-end has "models" directory to mimic the "models" workspace
- v0.4 Create a macro to recreate a struct that includes SurrealDB ID.

### Changed

- In an attempt to import SurrealDB's `Thing` struct, updated Leptos to 0.6.11
- Removed "models" workspace from front-end because incorporating SurrealDB caused WASM compilation errors.
- Moved CSS styling out of index.html file and into its own style sheet. I could not find an easy to use tool for modular CSS.

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
