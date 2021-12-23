# Malogany

![Sample of Malogany's output](screenshot.png)

<p align="center">
<b>Simple tree/hierarchical logging library</b><br>
<i>(Tree > Mahogany) + Logging => Malogany</i>
</p>

## Usage

Add the following to your `Cargo.toml`

```toml
[dependencies]
log = "0.4"
malogany = "0.1"
```

Initalize Malogany with the max log level

```rust
malogany::init(Level::Trace).unwrap();
```

For normal messages, use the `info!`, `warn!`, `error!` and `debug!` macros from the `log` crate. To create a branch, use `enter_branch` and `exit_branch`:

```rust
malogany::enter_branch("ident");

// any log messages will be nested within the ident branch
// you can keep nesting with `enter_branch`/`exit_branch`

malogany::exit_branch();
```

Branches are only rendered in debug builds for performance reasons. Future versions of this crate will make this customizable.

## Example

The screenshot above is the output of `example/basic.rs`. Try it for yourself with `cargo run --example basic`.
