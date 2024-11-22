# RCLI

rcli is a rust cli tool.

## Usage

```sh
rcli on  feature/refactor is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base took 10.8s
➜ cargo run -- csv -i assets/juventus.csv --format json
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/rcli csv -i assets/juventus.csv --format json`


rcli on  feature/refactor [!] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base took 5.6s
➜ cargo run -- genpass -l 32
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/rcli genpass -l 32`
aK54VX@m_bwhzFdWoxud^R7by9%4787V
Password strength: Four


rcli on  feature/refactor [!] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ cargo run -- base64 encode
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rcli base64 encode`
hello!
aGVsbG8hCg==


rcli on  feature/refactor [!] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ cargo run -- base64 encode --format urlsafe -i Cargo.toml > tmp.b64
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rcli base64 encode --format urlsafe -i Cargo.toml`

rcli on  feature/refactor [!?] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ cargo run -- base64 decode --format urlsafe -i tmp.b64
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/rcli base64 decode --format urlsafe -i tmp.b64`
[package]
name = "rcli"


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


rcli on  feature/refactor is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base took 39.6s
➜ cargo run -- text sign -k fixtures/ed25519.sk --format ed25519
   Compiling rcli v0.1.0 (/Users/qiaopengjun/Code/rust/rcli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.70s
     Running `target/debug/rcli text sign -k fixtures/ed25519.sk --format ed25519`
hello!
gATe_c_7hUS-QnuGnVyIEyB1ivnzcFKNOqrDf4L_WGg-UUXhWt2nSa-Fg-Du4yueBbecPi5xvSWkeFwmyDH2AQ

rcli on  feature/refactor is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base took 7.4s
➜ cargo run -- text verify -k fixtures/ed25519.pk --format ed25519 --sig gATe_c_7hUS-QnuGnVyIEyB1ivnzcFKNOqrDf4L_WGg-UUXhWt2nSa-Fg-Du4yueBbecPi5xvSWkeFwmyDH2AQ
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/rcli text verify -k fixtures/ed25519.pk --format ed25519 --sig gATe_c_7hUS-QnuGnVyIEyB1ivnzcFKNOqrDf4L_WGg-UUXhWt2nSa-Fg-Du4yueBbecPi5xvSWkeFwmyDH2AQ`
hello!
✓ Signature verified


rcli on  feature/refactor [!?] is 📦 0.1.0 via 🦀 1.82.0 via 🅒 base
➜ cargo run -- http serve
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/rcli http serve`
```
