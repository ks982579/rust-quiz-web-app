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

Check out [Using Rust and Leptos to build beautiful, declarative UIs | LogRocket.com](https://blog.logrocket.com/using-rust-leptos-build-beautiful-declarative-uis/).
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

I created my own middleware to check for user session cookie.
It is in the `AuthCookie` struct.

[OpenAPI 3.1 Specification | Swagger.io](https://swagger.io/specification/)

#### GET /api/v01/health-check

Create JSON objects for request and response.
Nothing too intense.

#### POST /api/v01/create-user

```yaml
openapi: 3.1.0
info:
  title: Create User API
  version: 0.2.0
  description: API for creating new users

servers:
  - url: https://kevsquizapp.com/api/v01/create-user

paths:
  /api/v01/create-user:
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

#### POST /api/v01/user-login

```yaml
openapi: 3.1.0
info:
  title: User Login API
  version: 0.2.0
  description: API for user authentication

servers:
  - url: https://kevsquizapp.com/api/v01/user-login

paths:
  /api/v01/user-login:
    post:
      summary: Log in user and create cookie
      description: Log a user into web application with provided credentials by setting token cookie
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/UserRequest"
      responses:
        "200":
          description: User successfully logged in
          headers:
            Set-Cookie:
              schema:
                type: string
                example: "session=abcd1234; Path=/; Secure; HttpOnly"
              description: Session cookie for authenitcated user
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
        - username
        - password
      properties:
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

#### GET /api/v01/check-login

```yaml
openapi: 3.1.0
info:
  title: Check User Authentication API
  version: 0.2.0
  description: API for user authentication and session management

servers:
  - url: https://kevsquizapp.com/api/v01/check-login

components:
  securitySchemes:
    cookieAuth:
      type: apiKey
      in: cookie
      name: sessionId
  schemas:
    GoodResponse:
      type: object
      properties:
        uuid
          type: string
          example: 1234-1234-1234
        name:
          type: string
          example: Lisa
        username
          type: string
          example: johndoe123
    ErrorResponse:
      type: object
      properties:
        msg:
          type: string
          example: Unknown Error

paths:
  /api/v01/check-login:
    get:
      summary: Check user login status and retrieve user data
      description: Verify user's session cookie and return user data if authenticated
      responses:
        "200":
          description: User is successfully authenticated
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/GoodResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
```

#### GET /api/v01/user-logout

```yaml
openapi: 3.1.0
info:
  title: Log out user from application
  version: 0.2.0
  description: API for logging a user out of the quiz application

servers:
  - url: https://kevsquizapp.com/api/v01/user-logout

components:
  securitySchemes:
    cookieAuth:
      type: apiKey
      in: cookie
      name: sessionId

paths:
  /api/v01/user-logout:
    get:
      summary: Logs user out of application
      description: Checks database for a session token and removes it, then sends reponse to browser to remove cookie
      responses:
        "200":
          description: User is no longer, or was never, logged in
          content:
            application/json: {}
        "500":
          description: Internal server error
          content:
            application/json: {}
```

#### /api/v01/quiz-nexus

```yaml
openapi: 3.1.0
info:
  title: Quiz Nexus
  version: 0.3.0
  description: Handling CRUD operations for quizzes

servers:
  - url: https://kevsquizapp.com/api/v01/quiz-nexus

components:
  securitySchemes:
    cookieAuth:
      type: apiKey
      in: cookie
      name: sessionId
  schemas:
    Thing:
      type: object
      properties:
        id:
          type: object
          properties:
            tb:
              type: string
              example: quizzes
            id:
              type: string
              example: String::<abc-123>
    QuizRequest:
      type: object
      properties:
        name:
          type: string
        description:
          type: string
    GoodResponse:
      type: object
      properties:
        id:
          type: object
          $ref: "#/components/schemas/Thing"
        name:
          type: string
          example: Algorithms
        description:
          type: string
          example: description of quiz
        author_id:
          type: string
    GoodResponseList:
      type: array
      items:
        type: object
        $ref: "#/components/schemas/GoodResponse"
    ErrorResponse:
      type: object
      properties:
        msg:
          type: string
          example: Unknown Error

paths:
  /api/v01/quiz-nexus:
    post:
      summary: Create new Quiz
      description: Given correct information, this will save a new quiz to the database
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/QuizRequest"
      responses:
        "200":
          description: (Should be 201) Indicates the quiz was successfully created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/GoodResponse"
        "400":
          description: can be returned if error validating information
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    get:
      summary: Fetch list of quizzes by user.
      description: Given correct information, this will return a list of quizzes owned by the user.
      responses:
        "200":
          description: Indicates the quizzes were successfully fetched
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/GoodResponseList"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    put:
      summary: Update quiz information
      description: Given correct information, this will update the provided quiz information.
      parameters:
        - in: query
          name: quiz
          required: true
          schema:
            type: string
          description: The raw Thing ID for the quiz record.
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/QuizRequest"
      responses:
        "200":
          description: Indicates the quiz was successfully updated
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/GoodResponse"
        "400":
          description: can be returned if error validating information
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "403":
          description: Forbidden (Quiz is not owned by user)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    delete:
      summary: Delete existing Quiz
      description: Given correct information, this will delete a quiz that is owned by the user.
      parameters:
        - in: query
          name: quiz
          required: true
          schema:
            type: string
          description: The raw Thing ID for the quiz record.
      responses:
        "200":
          description: Indicates the quizzes were successfully deleted
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/GoodResponse"
        "400":
          description: Bad Request (bad query parameter)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "403":
          description: Forbidden (Returned if user is not owner)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
```

#### /api/v01/question-forge

```yaml
openapi: 3.1.0
info:
  title: Question Forge
  version: 0.3.0
  description: Handling CRUD operations for questions

servers:
  - url: https://kevsquizapp.com/api/v01/question-forge

components:
  securitySchemes:
    cookieAuth:
      type: apiKey
      in: cookie
      name: sessionId
  schemas:
    Thing:
      type: object
      properties:
        id:
          type: object
          properties:
            tb:
              type: string
              example: quizzes
            id:
              type: string
              example: String::<abc-123>
    MultipleChoiceRequest:
      type: object
      properties:
        MultipleChoice:
          type: object
          properties:
            question:
              type: string
            hint:
              type: string
            answer:
              type: string
            choices:
              type: array
              items:
                type: string
    QuestionRequest:
      type: object
      properties:
        quiz_id:
          type: object
          $ref: "#/components/schemas/Thing"
        question:
          type: object
          oneOf:
            - $ref: "#/components/schemas/MultipleChoiceRequest"
    SurrealQuestionMC:
      type: object
      properties:
        id:
          type: object
          $ref: "#/components/schemas/Thing"
        question:
          type: string
          example: Algorithms
        hint:
          type: string
        author_id:
          type: string
        parent_quiz:
          type: object
          $ref: "#/components/schemas/Thing"
        answer:
          type: string
        choices:
          type: array
          items:
            type: string
    AllQuestions:
      type: object
      properties:
        mc:
          type: array
          items:
            type: object
            $ref: "#/components/schemas/SurrealQuestionMC"
    ErrorResponse:
      type: object
      properties:
        msg:
          type: string
          example: Unknown Error

paths:
  /api/v01/question-forge:
    post:
      summary: Create new Question
      description: Given correct information, this will save a new question for a given quiz
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/QuestionRequest"
      responses:
        "201":
          description: Indicates the question was successfully created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/SurrealQuestionMC"
        "400":
          description: can be returned if error validating information
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    get:
      summary: Get Questions for a quiz
      description: Given correct information, this will return a list of questions for a quiz.
      parameters:
        - in: query
          name: quiz
          required: true
          schema:
            type: string
          description: The raw Thing ID for the quiz record.
      responses:
        "200":
          description: Indicates the questions were successfully fetched
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/AllQuestions"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    put:
      summary: Update existing Question
      description: Given correct information, this will update a question for a given quiz.
      parameters:
        - in: query
          name: quest
          required: true
          schema:
            type: string
          description: The raw Thing ID for the question record.
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/QuestionRequest"
      responses:
        "201":
          description: Indicates the question was successfully created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/SurrealQuestionMC"
        "400":
          description: can be returned if error validating information
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "403":
          description: Forbidden (Returned if user is now owner)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
    delete:
      summary: delete a question
      description: Given correct information, this will delete a question.
      parameters:
        - in: query
          name: quest
          required: true
          schema:
            type: string
          description: The raw Thing ID for the question record.
      responses:
        "200":
          description: Indicates the questions were successfully fetched
          content:
            application/json:
              schema:
                oneOf:
                  - $ref: "#/components/schemas/SurrealQuestionMC"
        "401":
          description: Unauthorized (No or Invalid session cookie)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "403":
          description: Forbidden (Returned if user is not owner)
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
        "500":
          description: Internal server error
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorResponse"
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
