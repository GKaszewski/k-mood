use domain::attachment::{MediaKind, MediaRef, PhotoId, VoiceMemoId};

use crate::responses::{MediaIdResponse, OwnedMediaResponse};

impl From<PhotoId> for MediaIdResponse {
    fn from(id: PhotoId) -> Self {
        Self { id: id.value() }
    }
}

impl From<VoiceMemoId> for MediaIdResponse {
    fn from(id: VoiceMemoId) -> Self {
        Self { id: id.value() }
    }
}

impl From<MediaRef> for OwnedMediaResponse {
    fn from(media: MediaRef) -> Self {
        let path = match media.kind() {
            MediaKind::Photo => "photos",
            MediaKind::VoiceMemo => "voice-memos",
        };

        Self {
            url: format!("/api/v1/media/{path}/{}", media.id()),
            id: media.id().to_string(),
            kind: media.kind().name().to_string(),
        }
    }
}
