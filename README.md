# OpenAI Chat

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

### Dependencies

```bash
npm install
cargo install cargo-watch
```

### Environment Variables

Copy `.env.example` to `.env` and update the variables.

## Usage

### Production

```bash
npm run dev
cargo run
```

### Development

```bash
npm run watch
cargo watch -x run
```
