# Every refusal is one shape, and the spec says so

A client talking to this API had to parse four different failure bodies: `{"error":{"code","message"}}` from a handler, `{"error":"..."}` from the auth extractor, the same again from the path-id extractor, and axum's plain text when a JSON body would not parse. None of the four appeared in the OpenAPI document, which described zero refusals across sixty-four operations.

Login and refresh — the first two calls any client makes — returned `Json<serde_json::Value>` and were documented with a description and no schema, so a generated client received an untyped blob from the endpoints it needs most.

## Consequences

`ErrorResponse` lives in `api-types` beside every other payload, and `refuse` is the single function that builds one. The auth and path-id rejections call it; `Body` and `Params` replace axum's `Json` and `Query` extractors so a malformed request is refused in the same shape as everything else; the rate limiter's 429 uses it too.

A scope refusal is now 403 rather than 401. The distinction matters to a client: 401 means the credential is not valid, and retrying with the same one is pointless; 403 means it is valid and insufficient, and the answer is a token minted with more.

Every operation declares its refusals, and four contract tests hold the line: every success carries a schema, every operation documents a 4xx, every guarded operation documents both 401 and 403, and every refusal references `ErrorResponse` and nothing else. A new endpoint that skips this fails the suite rather than shipping undocumented.

## The api-types crate says what each type is for

The crate now has three input-and-output modules rather than two: `requests` for bodies a client sends, `params` for query strings, `responses` for bodies it receives. Query parameters were previously mixed among request bodies, which made "what can a client send here" a question about the type's name rather than its module.

Every type carries the suffix of its kind, with no exceptions left — `MoodFrequency` and `ErrorDetail` were the last two, and both were nested payload parts whose names claimed otherwise.

The SPA's zod schemas now mirror the server's type names one for one. That is not cosmetic: aligning them surfaced that `AuthTokenResponse` had modelled `user` as optional to cover both login and refresh, so the SPA had no way to notice the server dropping a field from one of them.
