## Build tools & versions used

- cargo 1.92.0 (344c4567c 2025-10-21)
- rustc 1.92.0 (ded5c06cf 2025-12-08)
- sqlx-cli 0.8.6 (cargo install sqlx-cli@0.8.6)

## Steps to run the app

Create a `.env` file with the following content:
```
DATABASE_URL="sqlite://nodes.sqlite"
SERVER_ADDRESS="127.0.0.1"
SERVER_PORT=8080
```

and then run:

- sqlx database create
- cargo run

## What was the reason for your focus? What problems were you trying to solve?
I was mostly focusing on making the codebase as simple as possible and not overcomplicating it. I believe this is a good practice to make things conceptually simple and easy to understand, this is also the reason I didn't add many comments in the code as they are well-organized and self-explanatory.

## How long did you spend on this project?
Almost 5 hours. From 20:16 to 1:29. I was kind of watching a Ghibli movie with my kid and I had to take a break to put him to sleep, so... I would say a little less than 5h :)

## Did you make any trade-offs for this project? What would you have done differently with more time?
I don't think I made any trade-offs, but I could have used more time to create tests and profile the server to see if it was performant enough.

## What do you think is the weakest part of your project?
I think the weakest part of my project is the error handling. It's a monolithic error type that is quite unnecessary. For example: some errors will never happen on the web server, so there is no point in implementing ResponseError for some of them. I also used SQLite for the database, which is good for reproducing the project, but it's not the best choice for a production environment. I would create a trait for the database and rework it for a more production-ready version, probably using Postgres.
