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

### Frontend

Check out [Using Rust and Leptos to build beautifyl, declarative UIs | LogRocket.com](https://blog.logrocket.com/using-rust-leptos-build-beautiful-declarative-uis/).
The `trunk` crate is a bundler.
It will compile Rust to WASM and bundle in the `frontend/dist` directory
Since it is SPA, going to follow "component-based architecture.
There is a book "React Application Architecture for Production" by Alan Alickovic
that begins a project structure overview section on page 28.

### Backend

Per [REST API Architectural Constraints | geeksforgeeks.org](https://www.geeksforgeeks.org/rest-api-architectural-constraints/)
the backend will follow the RESTful API architecture, or very close too.
Ideally, the project will closely follow the URL structure to keep file organized
and their location predictable.

[OpenAPI 3.1 Specification | Swagger.io](https://swagger.io/specification/)

#### GET /health-check

Create JSON objects for request and response.
Nothing too intense.

#### POST /create-user

```yaml
openapi: 3.1.0
info:
  title: Create User API
  version: 0.1.0
  description: API for creating new users

servers:
  - url: http://api.example.com/v1

paths:
  /users:
    post:
      summary: Create a new user
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/UserRequest"
      responses:
        "201":
          description: User created successfully
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/UserResponse"
        "400":
          description: Bad request (invalid input data)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/UserResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/UserResponse"

components:
  schemas:
    UserRequest:
      type: object
      required:
        - name
        - username
        - password
      properties:
        name:
          type: string
          example: John Doe
        username:
          type: string
          example: johndoe123
        password:
          type: string
          format: password
          example: at_least_6_chars
    UserResponse:
      type: object
      properties:
        msg:
          type: string
          example: Unknown Error
```

---

Unsure where to put this for now, but the session token in the database looks like:

```json
[
  {
    expiry: '2024-06-14T17:26:06.403Z',
    id: sessions:Amxy66...LOL,
    token: '{"user_id": "\\"ae51-...-5e37c71b835c\\""}'
  }
]
```
