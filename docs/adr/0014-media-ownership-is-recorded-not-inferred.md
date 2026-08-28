# Media ownership is recorded, not inferred from the entries that wear it

`MediaStoragePort` is a blob store — local filesystem or S3 behind `object_store` — and blobs have no owner. `DELETE /api/v1/media/photos/{id}` therefore authenticated the caller and then threw the id away, so any account could delete any other account's photo or voice memo by naming its uuid.

Ownership could not be checked because nothing recorded it. The only trace was the `entry_photos` join, and that trace does not exist for the case the endpoint is for: a photo uploaded and then dropped before the entry was ever saved.

## Considered Options

- **Infer the owner from `entry_photos` joined to `mood_entries`** — free, and wrong for exactly the uploads the delete endpoint serves. A just-uploaded blob has no entry, so it would be undeletable or unprotected, and which of the two depends on how the absence is read.
- **Namespace the object key by user (`photos/{user_id}/{uuid}`)** — ownership becomes structural and needs no table. It also puts the account id in every media URL the SPA renders, and rewrites every `get`/`delete` signature so the caller must already know the owner, which is the thing being established.

## Consequences

A `media_owners` table records `(kind, media_id, user_id)` at upload, behind a new `MediaOwnershipPort`. The blob store stays ignorant of users, which is correct: object stores do not do authorization.

`MediaRef` carries the kind alongside the uuid so one port serves both photos and voice memos without six near-duplicate methods. Because the kind is part of the key, a `PhotoId` and a `VoiceMemoId` sharing a uuid are still two different objects, and claiming one does not claim the other.

Media restored from a backup is claimed for the restoring account, so a restore does not produce blobs nobody owns.

The migration backfills owners from `entry_photos` and `entry_voice_memos`. Blobs stranded by an abandoned upload predating this table have no owner and cannot be deleted through the API — they are unreachable rather than dangerous, and a sweep for them is work this document does not do.

## The read path is still unauthenticated

`GET /api/v1/media/photos/{id}` takes no bearer token, because the SPA renders photos in `<img src>` and cannot attach one. Access rests on the uuid being unguessable. That is a deliberate capability URL, not an oversight, and it is recorded here because it is indistinguishable from one in the handler. Closing it means fetching blobs through XHR and object URLs, or a cookie scoped to the media routes, and neither is worth doing until media is shared beyond the account that uploaded it.
