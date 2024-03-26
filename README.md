# zoidberg 🦀

![test](https://i.pinimg.com/originals/5b/68/cc/5b68cceb2221337701855187654dc5b2.gif)

Simple rust script to go through your linkedin history and find the recruiters that have provided you with relevant job specs.

## Prerequisite

- Have rust and cargo installed on your machine:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## How to use

- Clone the repo
- rename .env.example to .env and fill in the required fields
- Download your "messages.csv" file from linkedin, you can do so here: https://www.linkedin.com/mypreferences/d/download-my-data
  (only select messages)
- Place the file in src/data/messages.csv
- Run the following command:

```
cargo run
```
