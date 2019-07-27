# bunq

An easy-to-use async Bunq API client

If you do not bank with Bunq, this is unlikely to be of use to you.

## Status: Pre-alpha

Does not yet work.

[ ] End-to-end requests
[x] Request signing
[ ] HTTP Client
[ ] Rate limiting
[ ] Response signature checking
[ ] API modelling
[ ] Signing key generation
[ ] Creating sandbox api keys
[ ] Offline tests
[ ] Online tests
[ ] Documentation / Guides

## Synopsis

```rust
use std::result::Result;
use bunq::client::{self, Client};
fn go() -> Result {
  let mut config = client::Config::default_sandbox(); // sandbox connection
  let client = Client::new(config)?;
}

```


## Rust Support

Currently requires nightly features:

* `fixed_size_array`

## Community + Contributions

Issues and pull requests gladly accepted.

Don't be a dick. Proper CoC to follow when the library is more mature.

## Copyright and License

Copyright (c) 2019 James Laver

This software is free and open source software licensed under the
terms of the Mozilla Public License (MPL) 2.0

