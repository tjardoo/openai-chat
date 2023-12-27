# OpenAI Chat

## Introduction

OpenAI Chat is a web application that allows you to interact with the [OpenAI API](https://beta.openai.com/). The application is build in [Rust](https://www.rust-lang.org/) and [Vue](https://vuejs.org/). The chat history is stored in a [MySQL](https://www.mysql.com/) database.

You can easily start a new chat or continue an existing chat via the links in the sidebar. You can choose the engine (model) for each chat individually. When a code snippet is detected, it will automatically be highlighted using [Highlight.js](https://highlightjs.org/).

Each new message will contain all previous messages of the chat - therefore the tokens used increases each time you send a new message. This is visible on the right-side of the timestamp below the message.

![Screenshot](docs/screenshot.png)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [MySQL](https://dev.mysql.com/downloads/installer)

### Dependencies

Install the following dependencies:

```bash
cargo install sqlx-cli

cargo install cargo-watch
```

### Setup

Run the following commands:

```bash
sqlx migrate run

cd client && npm install
```

Copy `.env.example` to `.env` and update the environment variables.

## Usage

### Production

```bash
cd client && npm run build

cargo run
```

And visit [http://localhost:3000](http://localhost:3000).

### Development

```bash
cd client && npm run watch

cargo watch -x run
```

And visit [http://localhost:5173](http://localhost:5173).

Optional commands:

```bash
sqlx migrate revert

sqlx database reset
```

#### Currently supported languages

- javascript
- php
- bash
- css
- json
- sql
- rust
- python

You can add more languages in `client\src\main.ts`.
