# Symmetric proxy re-encryption

A Rust implementation of the paper *Realizing Proxy Re-encryption in the Symmetric World* (https://link.springer.com/chapter/10.1007/978-3-642-25327-0_23).

---

## Disclaimer

I am not part of the team that wrote the paper implemented here, and I have no connection with them.

## Running the code

First install the nightly toolchain (needed for the `generic_const_exprs` feature):

```bash
rustup toolchain install nightly
```

Then you can run the code with:

```bash
cargo +nightly run
```

Or the tests with:

```bash
cargo +nightly test
```

## Functions

- `Key::generate()`
> Generate a new encryption / decryption key.

- `ReEncryptionKey::generate(old_key, new_key)`
> Generate a re-encryption key from `old_key` to `new_key`.

- `encrypt(message, key)`
> Encrypt `message` with `key`.

- `decrypt(encrypted, key)`
> Decrypt `encrypted` with `key`.

- `re_encrypt(encrypted, re_encryption_key)`
> Re-encrypt `encrypted` with `re_encryption_key`.


## Example

```rust
use symmetric_pre::{decrypt, encrypt, re_encrypt, Key, ReEncryptionKey};

let message = b"les sanglots longs des violons !";

println!("message: {:?}", message);
let key1 = Key::generate();
let encrypted = encrypt(message, &key1);
println!("encrypted: {:?}", encrypted);
let decrypted = decrypt(&encrypted, &key1);
println!("decrypted: {:?}", str::from_utf8(&decrypted).unwrap());

let key2 = Key::generate();
let re_encryption_key = ReEncryptionKey::generate(&key1, &key2);
let re_encrypted = re_encrypt(&encrypted, &re_encryption_key);
println!("re_encrypted: {:?}", re_encrypted);
let re_decrypted = decrypt(&re_encrypted, &key2);
println!("re_decrypted: {:?}", str::from_utf8(&re_decrypted).unwrap());
```