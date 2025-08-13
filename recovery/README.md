# 🦀 Recovery

Trait and derive macros to declare how errors should be retried.

## What it Does

This crate exports a trait that returns how an error should be retried:

```rust,ignore
pub trait Recovery {
    fn recovery(&self) -> RecoveryStrategy;
}
```

with:

* `RecoveryStrategy::Auto` means the error is temporary, the operation should be tried again. Examples for this are connectivity issues, HTTP throttling or SERVICE_UNAVAILABLE, etc.
* `RecoveryStrategy::Manual` means the operation might be recoverable, but shouldn't be done automatically. An example is HTTP UNAUTHORIZED errors which might indicate a configuration issue, but should only be retried once the underlying issue has been fixed.
* `RecoveryStrategy::Never` is for errors that are definitely fatal, such as invalid inputs.

## How to Use

You can implement `Recovery` for any error type. If your error is an `enum` (most likely) you can use `#[derive(Recovery)]`:

```rust
use recovery::Recovery;

#[derive(Recovery)]
pub enum MyError {
    #[recovery(auto)]
    Temporary,
    #[recovery(manual)]
    Maybe,
    #[recovery(never)]
    Fatal,
}
```

The macro also supports a default option if none is defined for one of the variants:

```rust
use recovery::Recovery;

#[derive(Recovery)]
#[recovery(never)]
pub enum Nah {
    Bad1,
    Bad2,
    Bad3,
    #[recovery(manual)]
    Maybe,
}
```

Finally, you can use `#[recovery(transparent)]` to use an internal variable's `Recovery` implementation:

```rust
use recovery::Recovery;

#[derive(Recovery)]
pub enum Outer {
    #[recovery(transparent)]
    Inner(Inner),
}

#[derive(Recovery)]
pub enum Inner {
    #[recovery(manual)]
    Maybe,
}
```
