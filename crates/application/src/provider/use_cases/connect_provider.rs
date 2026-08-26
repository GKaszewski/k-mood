use std::sync::Arc;

use domain::ports::ProviderConnectionCommandPort;
use domain::provider::{CredentialCipher, ProviderConnection};

use crate::errors::ApplicationError;

use super::super::commands::ConnectProviderCommand;

pub struct Deps {
    pub command: Arc<dyn ProviderConnectionCommandPort>,
    pub cipher: Arc<dyn CredentialCipher>,
}

#[tracing::instrument(skip(deps, cmd), fields(provider = cmd.provider.value()))]
pub async fn execute(cmd: ConnectProviderCommand, deps: &Deps) -> Result<(), ApplicationError> {
    let credential = deps.cipher.encrypt(&cmd.credential)?;
    let connection = ProviderConnection::new(cmd.user_id, cmd.provider, credential);

    deps.command.save(&connection).await?;

    Ok(())
}
