use super::{PhotoId, VoiceMemoId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaKind {
    Photo,
    VoiceMemo,
}

impl MediaKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Photo => "photo",
            Self::VoiceMemo => "voice_memo",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "photo" => Some(Self::Photo),
            "voice_memo" => Some(Self::VoiceMemo),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MediaRef {
    kind: MediaKind,
    id: uuid::Uuid,
}

impl MediaRef {
    pub fn to_photo(&self) -> Option<PhotoId> {
        match self.kind {
            MediaKind::Photo => Some(PhotoId::from_uuid(self.id)),
            MediaKind::VoiceMemo => None,
        }
    }

    pub fn to_voice_memo(&self) -> Option<VoiceMemoId> {
        match self.kind {
            MediaKind::VoiceMemo => Some(VoiceMemoId::from_uuid(self.id)),
            MediaKind::Photo => None,
        }
    }

    pub fn kind(&self) -> MediaKind {
        self.kind
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }
}

impl From<&PhotoId> for MediaRef {
    fn from(id: &PhotoId) -> Self {
        Self {
            kind: MediaKind::Photo,
            id: id.value(),
        }
    }
}

impl From<&VoiceMemoId> for MediaRef {
    fn from(id: &VoiceMemoId) -> Self {
        Self {
            kind: MediaKind::VoiceMemo,
            id: id.value(),
        }
    }
}

impl std::fmt::Display for MediaRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.kind.name(), self.id)
    }
}
