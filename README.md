# Web Desktops

Web desktops leverages your browser's built-in screen capture functionality to
stream your desktop between browsers over WebRTC.

Currently, the base functionality has been implemented and you can stream your
desktop over the internet however, the app's appearance is incomplete and there
are various features planned.

## Roadmap

- Finish polishing the frontend
	- First release!
- Options menu
- Remote-desktop functionality
	- Will stream mouse and keyboard events over WebRTC
	- Can be toggled in options

## Building

> NOTE: Do not run this project in a production environment as debug flags are
currently still enabled. They can be disabled in the `Cargo.toml` file.

`cargo-leptos` is used to build the project:

```sh
cargo install --locked cargo-leptos
```

See the [cargo-leptos](https://github.com/leptos-rs/cargo-leptos/) repository
for more details.

<br>

Run the project with:

```sh
cargo leptos --release serve
```
