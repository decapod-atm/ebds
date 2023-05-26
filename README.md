# EBDS Serial Protocol

This crate implements the EBDS serial protocol messages, and related types for communication with bill acceptor unit devices.

The currently supported messages are implemented in the various modules in this crate, along with some common types used across multiple messages.

If adding a new message, please follow the existing pattern of placing `...Command` (host-initiated) messages in `<message-type>/command.rs` files, and `...Reply` (device-initiated) messages in `<message-type>/reply.rs` files.

There are some exceptions to the general rule, e.g. when the types in the documenation do not follow the `Command/Reply` naming convention.

In those cases, the suffix is omitted to aid in readability when comparing with the EBDS specification.

## Macros

Some simple macros exist for implementing traits over the various message types. All message types should implement `MessageOps`, and all reply types should implement `OmnibusReplyOps`.

`MessageOps` can be implemented with the helper macro `impl_message_ops!`, e.g. for a new `SomeNewReply` message:

```rust
use crate::impl_message_ops;

pub struct SomeNewReply {
    // For the example, we are just using a number for the length.
    // In real implementations, please add a constant to the `len` module.
    buf: [u8; 11],
}

impl_message_ops!(SomeNewReply);
```

This will implement the `MessageOps` trait for `SomeNewReply`, and provide all of the associated functions. Traits are how Rust does polymorphism, similar to Go's `interface` and C++'s `template`, with important differences.

All of the macro implementations live in `src/macros.rs`.

## Using with `std`

This library is `no-std` compatible by default. To use `std`-only features, add the `std` feature to the dependency:

```toml
ebds = { version = "0.1", features = ["std"] }
```
