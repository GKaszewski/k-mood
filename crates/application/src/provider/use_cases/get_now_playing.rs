use std::sync::Arc;

use domain::errors::DomainError;
use domain::ports::{NowPlayingPort, ProviderConnectionQueryPort, RecordingLookupPort};
use domain::provider::{CredentialCipher, ProviderName};
use domain::song::Song;
use domain::user::UserId;

use crate::errors::ApplicationError;

pub struct Deps {
    pub query: Arc<dyn ProviderConnectionQueryPort>,
    pub cipher: Arc<dyn CredentialCipher>,
    pub now_playing: Arc<dyn NowPlayingPort>,
    pub recordings: Arc<dyn RecordingLookupPort>,
}

#[tracing::instrument(skip(deps))]
pub async fn execute(user_id: UserId, deps: &Deps) -> Result<Option<Song>, ApplicationError> {
    let provider = ProviderName::new(deps.now_playing.provider())?;

    let connection = deps
        .query
        .find_by_user_and_provider(&user_id, &provider)
        .await?
        .ok_or_else(|| {
            DomainError::NotFound(format!("no {} connection for this user", provider.value()))
        })?;

    let credential = deps.cipher.decrypt(connection.credential())?;

    let asked = deps.now_playing.now_playing(&credential).await;

    let Some(song) = unreachable_is_not_playing(asked, &provider) else {
        return Ok(None);
    };

    Ok(Some(enrich(song, deps).await))
}

fn unreachable_is_not_playing(
    asked: Result<Option<Song>, DomainError>,
    provider: &ProviderName,
) -> Option<Song> {
    match asked {
        Ok(playing) => playing,
        Err(error) => {
            tracing::warn!(
                provider = provider.value(),
                %error,
                "could not ask what is playing, so nothing is offered"
            );

            None
        }
    }
}

async fn enrich(song: Song, deps: &Deps) -> Song {
    if song.recording_id().is_some() {
        return song;
    }

    let found = deps
        .recordings
        .find_recording(song.title().value(), song.artist().value())
        .await;

    match found {
        Ok(Some(recording_id)) => Song::from_persistence(
            song.title().clone(),
            song.artist().clone(),
            song.album().cloned(),
            Some(recording_id),
        ),
        _ => song,
    }
}
