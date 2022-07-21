# Releases

Run `./release.sh <version>` to update `Cargo.toml`, tag and push to upstream. If your git remote isn't named `upstream`, set the `REMOTE` environment variable to the correct value.

Example:

```sh
REMOTE=origin ./release.sh 0.2.0
```

Once the tag is pushed, GH actions should publish the crate to crates.io.

