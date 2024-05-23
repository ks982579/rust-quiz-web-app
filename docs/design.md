# Design

## High Level Directory Structure

Being a full-stack web application, having "frontend" and "backend" directories is common.
This project is a Cargo workspace.
However, the frontend and backend should be built in a way such that the
frontend can be swapped with another technology, like React or Svelte.
Cargo workspace makes builds for this particular stack easier.

## Model-View-Presenter

[MVC | developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Glossary/MVC)
gives a great overview of the MVC architecture / design pattern.
It outlines that the Model directly communicates with the View.
This application will not have such a tight integration between the View and Model.
As such, a VMP architectural design pattern is more appropriate.
[Model-View-Presenter Design Pattern | toughgfx.com](https://support.touchgfx.com/4.20/docs/development/ui-development/software-architecture/model-view-presenter-design-pattern)
is a good resource for explaining this pattern

This workspace has 3 members:

- frontend - the View, handling user events and updating the UI.
- backend - the Presenter, handling communication between the View and Model.
- models - the Model, set of APIs to communicate with database.

[MVC Design Pattern | geeksforgeeks.org](https://www.geeksforgeeks.org/mvc-design-pattern/)
Also shows the MVC more like the MVP.
May need more consideration.

### Backend

Per [REST API Architectural Constraints | geeksforgeeks.org](https://www.geeksforgeeks.org/rest-api-architectural-constraints/)
the backend will follow the RESTful API architecture, or very close too.
Ideally, the project will closely follow the URL structure to keep file organized
and their location predictable.

### Frontend

Check out [Using Rust and Leptos to build beautifyl, declarative UIs | LogRocket.com](https://blog.logrocket.com/using-rust-leptos-build-beautiful-declarative-uis/).
