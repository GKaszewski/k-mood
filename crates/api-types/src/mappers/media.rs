use domain::attachment::{PhotoId, VoiceMemoId};

use crate::responses::MediaIdResponse;

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
