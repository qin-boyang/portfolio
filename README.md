# Developer portfolio

A terminal-inspired portfolio built with Rust and Dioxus for static hosting on GitHub Pages.

## Run locally

Install the Dioxus CLI, then start the web app:

```sh
cargo install dioxus-cli --version 0.7.0 --locked
dx serve --platform web
```

## Deploy

Push to `main` or `master` to build and deploy through GitHub Actions. In the repository settings, set **Pages → Build and deployment → Source** to **GitHub Actions**. The site is configured for the `/portfolio` project path in `Dioxus.toml`; update `base_path` if the repository name changes.

## Personalize

Edit the introduction, project cards, about text, and contact address in `src/main.rs`. Replace `you@example.com` with your email before publishing.
