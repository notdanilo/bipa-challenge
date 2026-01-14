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

## How long did you spend on this project?

## Did you make any trade-offs for this project? What would you have done differently with more time?

## What do you think is the weakest part of your project?

