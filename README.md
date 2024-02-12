# Erik Kurjak - Curriculum Vitae 🧑🏼‍💻

My personal website build with Rust (leptos), wasm bindgen, tailwind css for styling and deployed to cloud using fly.io with CI github actions.

## Design ✍️

Before I began implementing my website, I sketched the design, including typography and color palette, using Figma. It's not perfect, and it didn't need to be, as my main focus was on the technical aspects of the website.

<br />

<img src="https://leptoscv.s3.eu-central-1.amazonaws.com/figma-design-leptos-cv" height="500" alt="Figma CV Design">

<br />

## Why Rust - Leptos 🤷 ?

I've built a few applications in Rust and really fell in love with the language.

When I decided to create my portfolio for the fifth time, I was torn between using NextJS, Gatsby, Svelte, or other JavaScript frameworks. Then it hit me that Rust is compatible with wasm. After some research, I opted for `Leptos`.

It's a full-stack, isomorphic Rust web framework that utilizes fine-grained reactivity to construct declarative user interfaces.

It might be considered over-engineering at its best. 😄 Anyway, I went for it.

## Installing dependencies

Install `Rust` on your machine - make sure you have at least `1.70` version of Rust or newer.

https://www.rust-lang.org/tools/install

We need to install `node` dependencies for Tailwind and JS related work.

Make sure you have at least `Node 18` or newer.

Then run

`npm i`

If you don't have `cargo-leptos` installed you can install it with

`cargo install cargo-leptos`

## Running your project locally

`cargo leptos watch`

By default, you can access website at
`http://localhost:3000`

In other terminal window we need to turn on our watcher for `tailwind css`.

`npx tailwindcss -i ./input.css -o ./style/main.css --watch  `

This will build our css and each change to either rust file or adding a new tailwind class to rust file will trigger hot reload.

## Running your project locally with Docker

To run this website on your local docker you need to have Docker installed.

If you have docker installed run:

`docker-compose up --build`

After succesfull build you should see in your terminal

`leptos-cv-web-1  | listening on http://0.0.0.0:3000`

## CI pipeline

Each commit should have three prefixes:

**feat** - developing/creating someting new on website - this will trigger deployment

**fix** - fixing a bug on website - this will trigger deployment

**chore** - updating non coding stuff such as README, documentation etc... - this won't trigger deployment

Other prefixes or commit without prefix won't do deploy and will fail CI pipeline.

## TODO

- update favicon.ico
- update avatar image from png to webp and jpg support
- use wGPU for basic 2D/3D animation
- write an article about my journey using Rust for my portfolio

## 🔏 License

This project is [MIT](./LICENSE) licensed. You are within your rights to fork my porfolio website and use it as your own, although you should probably change my name to yours!
