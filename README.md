Shows broken code generation for async static factories.

Run `yarn build` to produce errors.

## Relevant Generated Chunks

**Async factory:**

```rust
napi::bindgen_prelude::execute_tokio_future(
    env,
    async move { Ok(MyNumber::future_4().await) },
    move |env, _ret| {
        _args_ref.drop(env);
        cb.factory("future4", _ret)
    },
)
```

Fails to compile because `cb` is not `Send`. `CallbackInfo::factory` requires an `env` (provided in this callback), but
also a `this`, which is not currently threaded through.

**Returning a result:**

```rust
napi::bindgen_prelude::execute_tokio_future(
    env,
    async move { MyNumber::maybe_future_4().await },
    move |env, _ret| {
        _args_ref.drop(env);
        cb.factory("maybeFuture4", _ret?)
    },
)
```

Fails to compile because `execute_tokio_future` does a layer of `Result` unwrapping, meaning that
`_ret` is now the value contained in `Ok`, and is no longer a `Result`, so the `?` does not apply.
