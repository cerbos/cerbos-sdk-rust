# Releases

Run `./release.sh <package> <version>` to update `Cargo.toml`, tag, and push to upstream. If your git remote isn't named `upstream`, set the `REMOTE` environment variable to the correct value.

Example:

```sh
REMOTE=origin ./release.sh cerbos 0.2.0
```

Once the tag is pushed, GH actions should publish the crate to crates.io.

> [!IMPORTANT]
> The `cerbos` crate depends on `if-struct-macro`, so publish `if-struct-macro` first!
