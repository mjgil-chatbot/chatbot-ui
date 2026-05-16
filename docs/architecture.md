# Architecture

## Overview

`chatbot-ui` is a host-independent `egui` UI library for browsing chat sessions, rendering messages, and composing prompts.

## Rendering Pipeline

- `src/message_list.rs` virtualizes the chat transcript and creates one row UI per visible message.
- `src/message.rs` renders each message shell, including author chrome, timestamps, copy actions, and assistant markdown delegation.
- Assistant markdown is rendered through the repo-owned `the-markdown-commons` dependency so parsing and rich-text behavior stay consistent across surfaces.

## Selection Behavior

- Inline assistant markdown is rendered as one selectable `egui` layout job per block instead of many sibling labels.
- Link destinations are still resolved from the inline fragment runs, but text selection now operates on the exact dragged character range within that single layout job.
- Hover underlines are painted from the hovered link's galley row geometry, so wrapped filename links stay fully underlined without spilling into trailing plain text or neighboring links.

## Release Packaging

- The active Cargo dependency is the sibling `../the-markdown-commons` checkout. A historical vendored copy is retained under ISC with its original `THIRD_PARTY_NOTICES`; it is not the active dependency in this manifest.
- CI validates formatting, build, and test execution with the checked-in lockfile to keep open source releases reproducible.
