# chatbot-ui

An egui UI library for browsing chat sessions, rendering messages, and composing
prompts. The host application implements the session, model, workspace, and UI
action traits exposed by this crate. This repository does not define a desktop
binary or own provider credentials and persistence.

## Layout and build

Place `the-markdown-commons` beside this checkout; Cargo uses the sibling path.
The historical vendored copy is retained as source material, but the package's
active dependency is the sibling crate.

```bash
cargo build --release --locked
cargo test --release --locked
```

Assistant markdown is delegated to the shared renderer. Message-list
virtualization, selectable text, copy actions, and hover/link behavior are
implemented in `src/message_list.rs` and `src/message.rs`. See
[architecture](docs/architecture.md) for the rendering boundary.

## Retained renderer tests

The historical vendored renderer has its own egui test suite:

```bash
cargo test --release --manifest-path vendor/the-markdown-commons/Cargo.toml --all-features --lib --bins --tests -- --test-threads=1
```

Reference-style links inside egui table cells currently retain their source text;
inline links are the workaround. The test requesting resolved table references is
explicitly ignored with that reason, as in the sibling renderer's known limitations.

## License

First-party UI code is licensed under [0BSD](LICENSE). The vendored Markdown
renderer retains [ISC](vendor/the-markdown-commons/LICENSE) and its
[third-party notices](vendor/the-markdown-commons/THIRD_PARTY_NOTICES).
