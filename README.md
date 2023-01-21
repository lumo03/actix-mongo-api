# actix-mongo-api

This project is a RESTful API built using Rust and the Actix-web framework, which connects to a MongoDB database. The API allows for CRUD operations on a "User" resource.

## Technology stack
- Rust: v1.52.1
- Actix-web: v4.2.1
- MongoDB: v2.3.1
- Serde: v1.0.152
- dotenv: v0.15.0
- futures: v0.3.25

## How to run
1. Clone the repository and navigate to the project directory
2. Create an .env file in the root of the project with the following key: `MONGOURI=<your_mongodb_uri>`
3. Run `cargo run` to start the server
4. The API will be accessible at `http://localhost:8080`


## API Endpoints

### Create User

> `POST /user`
>
> Body:
> ```json
> {
>   "name": "Demola Malomo",
>   "location": "Software Engineer",
>    "title": "software engineer"
> }
> ```
> Response:
> ```json
> {
>    "insertedId": {
>        "$oid": "63cc311ebdf12dc12a181f3e"
>    }
>}
> ```


### Get a User

> `GET /user/:id`


### Update a User

> `PUT /user/:id`
>
> Body:
> ```json
> {
>   "name": "John Doe",
>    "location": "New York",
>    "title": "Developer"
> }
> ```

### Delete a User

> `DELETE /user/:id`
