# Rusty Leaf Backend

This backend is written in rust and will be deployed with [Shuttle](https://shuttle.rs)

To run this project you need to install
- Rust
- Shuttle Runtime

- Login with your shuttle account
- Copy Mongodb URL and paste it to .env file as `MONGO_URI` (see .env.sample for more info)
- run `cargo shuttle run`


### Currently implemented Endpoints
- `GET /user/<username>` 
- `POST /user` - to create new user ( registering )
- `POST /login` - to login into the account ( gets JWT Token back as response )

