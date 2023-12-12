# OpenAI Chat

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

### Dependencies

```bash
cargo install sqlx-cli
cargo install cargo-watch
sqlx migrate run
```

```bash
npm install
```

### Environment Variables

Copy `.env.example` to `.env` and update the variables.

## Usage

### Production

```bash
npm run prod
cargo run
```

### Development

```bash
npm run watch
cargo watch -x run
sqlx migrate revert
sqlx database reset
```
