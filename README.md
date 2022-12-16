# snake-game

About
A web application (game) to play at snake, using TypeScript, Rust and WebAssembly.

Credit: https://github.com/Jerga99 and https://www.udemy.com/course/rust-webassembly-with-js-ts-the-practical-guide

<p align="center">
  <img src="https://github.com/federicobaldini/snake-game/blob/master/application.png" alt="login" />
</p>

## Requirements

wasm-pack build --target web

To successful run this code, you need to have Rust and Cargo installed on your Machine.

For the instalation guide [click here](https://www.rust-lang.org/learn/get-started).

## Getting started 

Just clone the repo and use cargo to run the code as shown below

```bash
    $ git clone https://github.com/federicobaldini/snake-game
    $ cd snake-game
    snake-game->$ wasm-pack build --target web
```

Then to start the web application 

```bash
    snake-game->$ cd server
```

Install the dependencies:

```
npm install
```

then start the development server:

```
npm run dev
```

Navigate to [localhost:3000](http://localhost:3000). You should see the app running.
