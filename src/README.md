## Build tools & versions used

```
rustc 1.81.0 (eeb90cda1 2024-09-04)
axum = "0.7.7"
chrono = "0.4.38"
reqwest = "0.12.8"
serde = { version = "1.0.210", features = ["derive"]}
serde_json = "1.0.128"
sqlx = { version = "0.8.2", features = ["sqlite", "macros", "runtime-tokio"] }
tokio = { version = "1.40.0", features = ["full"] }
```

## Steps to run the app
```
git clone
cargo run 
curl -XGET http://localhost:3000/nodes
```

## What was the reason for your focus? What problems were you trying to solve?

## How long did you spend on this project?
About 6 hours (1 hour a day, 5 hours another day)

## Did you make any trade-offs for this project? What would you have done differently with more time?

## What do you think is the weakest part of your project?
The database

## Is there any other information you’d like us to know?
