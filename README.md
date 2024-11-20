# RCLI

rcli is a rust cli tool.

## Usage

```sh
rcli on  feature/text [!?] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ cargo run -- text sign -k fixtures/ed25519.sk --format ed25519
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/rcli text sign -k fixtures/ed25519.sk --format ed25519`
hello!^D
KiS91TPswopDjf7v6oRjA9-U57SMuG9VYcpQ9XWh8vYsACFwLJ6anUG-Nw3ZCqJN_-Bdd67op5xd1cQ8pRayBQ

rcli on  feature/text [!?] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base took 7.9s
➜ cargo run -- text verify -k fixtures/ed25519.pk --format ed25519 --sig KiS91TPswopDjf7v6oRjA9-U57SMuG9VYcpQ9XWh8vYsACFwLJ6anUG-Nw3ZCqJN_-Bdd67op5xd1cQ8pRayBQ
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/rcli text verify -k fixtures/ed25519.pk --format ed25519 --sig KiS91TPswopDjf7v6oRjA9-U57SMuG9VYcpQ9XWh8vYsACFwLJ6anUG-Nw3ZCqJN_-Bdd67op5xd1cQ8pRayBQ`
hello!
true
```
