use std::sync::Arc;

use domain::attachment::{ContentType, MediaUpload};
use domain::errors::DomainError;
use domain::ports::MediaStoragePort;
use domain::testing::{FakeMediaStorage, InMemoryStore, test_user};

use application::errors::ApplicationError;
use application::media::use_cases::{delete_photo, delete_voice_memo, upload_photo};

fn an_upload() -> MediaUpload {
    MediaUpload::new(vec![1, 2, 3], ContentType::new("image/png").unwrap()).unwrap()
}

fn upload_deps(storage: Arc<FakeMediaStorage>, store: Arc<InMemoryStore>) -> upload_photo::Deps {
    upload_photo::Deps {
        storage,
        ownership: store,
    }
}

#[tokio::test]
async fn uploading_claims_the_photo_for_the_uploader() {
    let storage = Arc::new(FakeMediaStorage::holding_nothing());
    let store = Arc::new(InMemoryStore::new());
    let owner = test_user("alice");

    let photo_id = upload_photo::execute(
        owner.id().clone(),
        an_upload(),
        &upload_deps(storage.clone(), store.clone()),
    )
    .await
    .unwrap();

    let deps = delete_photo::Deps {
        storage: storage.clone(),
        ownership: store.clone(),
    };

    delete_photo::execute(photo_id.clone(), owner.id().clone(), &deps)
        .await
        .expect("the owner may delete their own photo");

    assert!(!storage.holds_photo(&photo_id));
}

#[tokio::test]
async fn a_stranger_cannot_delete_someone_elses_photo() {
    let storage = Arc::new(FakeMediaStorage::holding_nothing());
    let store = Arc::new(InMemoryStore::new());
    let owner = test_user("alice");
    let stranger = test_user("mallory");

    let photo_id = upload_photo::execute(
        owner.id().clone(),
        an_upload(),
        &upload_deps(storage.clone(), store.clone()),
    )
    .await
    .unwrap();

    let deps = delete_photo::Deps {
        storage: storage.clone(),
        ownership: store.clone(),
    };

    let refused = delete_photo::execute(photo_id.clone(), stranger.id().clone(), &deps).await;

    assert!(
        matches!(
            refused,
            Err(ApplicationError::Domain(DomainError::Forbidden(_)))
        ),
        "a stranger must be refused, got {refused:?}"
    );
    assert!(
        storage.holds_photo(&photo_id),
        "the blob must survive a refused delete"
    );
}

#[tokio::test]
async fn media_nobody_claimed_cannot_be_deleted_by_anyone() {
    let storage = Arc::new(FakeMediaStorage::holding_nothing());
    let store = Arc::new(InMemoryStore::new());
    let caller = test_user("alice");

    let orphan = storage.store_photo(an_upload()).await.unwrap();

    let deps = delete_photo::Deps {
        storage: storage.clone(),
        ownership: store.clone(),
    };

    let refused = delete_photo::execute(orphan.clone(), caller.id().clone(), &deps).await;

    assert!(
        matches!(
            refused,
            Err(ApplicationError::Domain(DomainError::NotFound(_)))
        ),
        "an unclaimed blob has no owner to check against, got {refused:?}"
    );
    assert!(storage.holds_photo(&orphan));
}

#[tokio::test]
async fn a_voice_memo_id_is_not_a_photo_id() {
    let storage = Arc::new(FakeMediaStorage::holding_nothing());
    let store = Arc::new(InMemoryStore::new());
    let owner = test_user("alice");

    let photo_id = upload_photo::execute(
        owner.id().clone(),
        an_upload(),
        &upload_deps(storage.clone(), store.clone()),
    )
    .await
    .unwrap();

    let memo_id = domain::attachment::VoiceMemoId::from_uuid(photo_id.value());

    let deps = delete_voice_memo::Deps {
        storage: storage.clone(),
        ownership: store.clone(),
    };

    let refused = delete_voice_memo::execute(memo_id, owner.id().clone(), &deps).await;

    assert!(
        matches!(
            refused,
            Err(ApplicationError::Domain(DomainError::NotFound(_)))
        ),
        "claiming a photo must not claim a voice memo that shares its uuid, got {refused:?}"
    );
}
