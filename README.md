# OpenAI Chat

## Introduction

OpenAI Chat is a web app that allows you to interact with the [OpenAI API](https://beta.openai.com/). The application is build in [Rust](https://www.rust-lang.org/) and [Vue](https://vuejs.org/). The chat history is stored in a [MySQL](https://www.mysql.com/) database.

You can easily start a new chat or continue an existing chat via the links in the sidebar. When a code snippet is detected, it will automatically be highlighted using [Highlight.js](https://highlightjs.org/). When enabled, it will automatically summarize the first message and use that as title for the chat.

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

The `OPENAI_API_KEY` is required to use the OpenAI API. You can get your API key [https://beta.openai.com/account/api-keys](https://beta.openai.com/account/api-keys).

The `OPENAI_CHAT_SUMMARY_ENABLED` is used to decide, whether to automatically summarize the first message and use that as title for the chat.

## Usage

### Production

```bash
cd client && npm run build

cargo run
```

And visit [http://localhost:3000](http://localhost:3000).

You can configure the URL and port in `.env`.

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
