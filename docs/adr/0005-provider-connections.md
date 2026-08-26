# Provider credentials are stored server-side, encrypted, behind one generic concept

Auto-filling what a User was listening to requires calling their music server. We store a per-User ProviderConnection — a Provider name plus a credential encrypted under a key from the environment — and the server makes the call.

## Considered Options

- **Client-side resolution, no credentials on the server** — the safer default, and preferred on security grounds: a leaked database would expose nothing new. Rejected because the SPA is a browser PWA and would hit CORS and mixed-content restrictions against a self-hosted music server, and each client would have to implement auto-fill separately.

## Consequences

Credentials are sealed with XChaCha20-Poly1305. Authenticated encryption means a tampered ciphertext is rejected rather than decrypting to rubbish, and a random 24-byte nonce stored alongside each ciphertext means the same credential never encrypts to the same bytes twice — so equal ciphertexts cannot reveal that two users share a password.

`EncryptedCredential` renders as `<redacted>` in `Debug`, so a credential cannot reach a log through a derived `Debug` on any type that holds one. The same redaction is applied to the command and request types that carry a plaintext credential in transit.

The database now holds a recoverable credential for another system. The classic Subsonic scheme computes a per-request token from the plaintext password, so the stored secret is password-equivalent. Encryption raises the bar against a stolen database file alone, but a self-hosted single-container deployment usually backs up environment and data together, in which case it buys little — this is accepted knowingly.

The domain does not know what a Provider is or how it is reached. It holds an opaque credential and a name; parsing, authenticating, and calling are entirely adapter concerns, which is what lets a second Provider with a different credential shape (OAuth, API key) be added without touching the domain.
