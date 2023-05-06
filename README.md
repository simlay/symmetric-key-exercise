# Prompt (slightly modified)

The following test can be implemented in any programming language. You will
have 5 business days to complete, but it's not expected that you spend more
than 3 hours on it.

The assessment involves writing 2 basic programs using a shared library and a
symmetrical key.  Both programs will implement a command-line-interface (CLI).
Program 1 will take a message and the key (or passphrase) as input, using the
key it will encrypt the message and output a success message.  Program 2 will
take key as input and output the decrypted message on success and error message
on fail (i.e. wrong key).  The cryptography logic will be contained in the
shared library to be used by both programs.  Company foobar encourages
creativity in solving problems.  Therefore, it is up to the candidate on how
the ciphertext is delivered to Program 2.  Code should be pushed to a public
git repo (GitHub, GitLab, BitBucket, etc) early in the development so company
foobar engineers can review commits.

The command line clients should provide the following:

```
$ <program1name> -k <key> -m <message>  # Initiate encryption for program 1
$ <program2name> -k <key>               # Decrypt the message and output the plaintext
```

Notes:
- You can use external packages/libraries.
- Cryptography algorithms to be used are at the candidate's discretion.
- Bonus for programs written in C/C++ or Go.
- Bonus for implementing CMAKE.
- Bonus for creating stories with weights during development.

# Implementation Notes

[![codecov](https://codecov.io/gh/simlay/symmetric-key-exercise/branch/main/graph/badge.svg?token=B8BF0N27WS)](https://codecov.io/gh/simlay/symmetric-key-exercise)

The prompt for this leaves a few details up to the candidate of this exercise
and so the candidate (Sebastian Imlay) has made a few executive decisions with
pros and cons of each.

Additional Project goals (some for self education):

- [X] Do not roll your own cryptography.
- [X] Project will be done in Rust
- [X] Cryptography dependencies are implemented in Rust
- [X] Cryptography [dependencies are
**reputable**](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305).
The RustCrypto group is responsible for [rustsec.org](https://rustsec.org/).
- [X] Ideally [cryptography dependencies are
audited](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305#security-notes).
- [X] Select a balance of usability and security. See subjection.
- [X] Reasonable tests and code coverage tools.

This project uses the [Chchat20 and
Poly1305 protols](https://datatracker.ietf.org/doc/html/rfc8439) for encryption and
decryption.

This choice was due to a few factors:
* An informational RFC authored by google
* The Rust crate is maintained by the RustCrypto github organization.
* The Rust crate has [had a security
audit](https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305#security-notes).
* Fancy name.

## Nonce trade-offs

A brief bit of research on [AEAD
Algorithms](https://en.wikipedia.org/wiki/Authenticated_encryption) yields the
question what to do about replay attacks for this prompt.

A common thing to do with things here is to use a
[nonce](https://en.wikipedia.org/wiki/Cryptographic_nonce). This results in an
encrypted message to be unique. This is important for network transmitted
encrypted messages as those could be replayed.

This exercise supports:
* Null nonces (cli argument `--no-nonce`) - encryption uses an array
(`Vec<u8>`) of all zeros to encrypt/decrypt. Usage of a null nonce is
**strongly** are not recommended as this is disregarding any bits of entropy.
* Generated nonces (cli argument `--generate-nonce`) - encryption displays the
nonce needed to decrypt in the `--nonce` argument. While I have done a
rudimentary amount of statistics on nonce generation, **This nonce generation
was not done by a Cryptograher**.
* Specified nonces (cli argument `--nonce`) up to 24 ASCII characters long.
This can be used for encryption and decryption and might be the most flexible
way to use this application.

# Usage (via cargo):

## Encryption:
```sh
$ cargo run --bin encrypt -- --key my-key-is-cool --message "what is this message" --generate-nonce
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/encrypt --key my-key-is-cool --message 'what is this message' --generate-nonce`
The nonce for this message was generated and it is: diobotcxodeklyzcocykpooa
```

## Decryption:
```sh
$ cargo run --bin decrypt -- --key my-key-is-cool --nonce diobotcxodeklyzcocykpooa
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/decrypt --key my-key-is-cool --nonce diobotcxodeklyzcocykpooa`
what is this message
```

## Help
```sh
$ cargo run --bin encrypt -- --help
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/encrypt --help`
Usage: encrypt [OPTIONS] --message <MESSAGE> --key <KEY> --no-nonce --generate-nonce --nonce <NONCE>

Options:
  -m, --message <MESSAGE>
          The message to be encrypted
  -k, --key <KEY>
          This is an encryption key. It must be less than 32 characters long
  -e, --encrypted-file <ENCRYPTED_FILE>
          This is the file which an message is encrypted/decrypted to/from [default: data.dat]
      --no-nonce
          **NOT RECOMMENDED:** This is a helper option to enable the nonce be all zeros. This results in the encrypted message be the same on every encryption and subject to a replay attacks
  -g, --generate-nonce
          This is a flag to enable a newly generated nonce on encryption. This will error when used on decryption
  -n, --nonce <NONCE>
          This is the string representation of a nonce as ascii characters up to 24 characters in length. This is required for decryption unless using the unrecommended --no-nonce feature
  -h, --help
          Print help
```
