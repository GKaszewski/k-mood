use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "k-mood API",
        version = "1.0.0",
        description = "Mood tracking journal API"
    ),
    modifiers(&SecurityAddon),
    paths(
        crate::handlers::auth::handle_login,
        crate::handlers::auth::handle_refresh,
        crate::handlers::auth::handle_logout,
        crate::handlers::entries::handle_create,
        crate::handlers::entries::handle_get,
        crate::handlers::entries::handle_list,
        crate::handlers::entries::handle_update,
        crate::handlers::entries::handle_delete,
        crate::handlers::entries::handle_filter_by_mood,
        crate::handlers::entries::handle_filter_by_activity,
        crate::handlers::entries::handle_stats,
        crate::handlers::entries::handle_calendar,
        crate::handlers::entries::handle_activity_correlation,
        crate::handlers::entries::handle_delete_by_date_range,
        crate::handlers::entries::handle_replace_activity,
        crate::handlers::activities::handle_create,
        crate::handlers::activities::handle_get,
        crate::handlers::activities::handle_list,
        crate::handlers::activities::handle_rename,
        crate::handlers::activities::handle_set_category,
        crate::handlers::activities::handle_archive,
        crate::handlers::activities::handle_unarchive,
        crate::handlers::activities::handle_delete,
        crate::handlers::users::handle_register,
        crate::handlers::users::handle_get_profile,
        crate::handlers::users::handle_update_profile,
        crate::handlers::users::handle_change_password,
        crate::handlers::users::handle_delete,
        crate::handlers::users::handle_clear_data,
        crate::handlers::reminders::handle_create,
        crate::handlers::reminders::handle_get,
        crate::handlers::reminders::handle_list,
        crate::handlers::reminders::handle_update,
        crate::handlers::reminders::handle_delete,
        crate::handlers::media::handle_upload_photo,
        crate::handlers::media::handle_upload_voice_memo,
        crate::handlers::media::handle_serve_photo,
        crate::handlers::media::handle_serve_voice_memo,
        crate::handlers::media::handle_delete_photo,
        crate::handlers::media::handle_delete_voice_memo,
        crate::handlers::import_export::handle_export,
        crate::handlers::import_export::handle_import,
        crate::handlers::push::handle_vapid_key,
        crate::handlers::push::handle_subscribe,
        crate::handlers::push::handle_unsubscribe,
        crate::handlers::push::handle_test,
    ),
    components(schemas(
        api_types::requests::CreateEntryRequest,
        api_types::requests::UpdateEntryRequest,
        api_types::requests::ListEntriesParams,
        api_types::requests::DateRangeParams,
        api_types::requests::ReplaceActivityRequest,
        api_types::requests::CreateActivityRequest,
        api_types::requests::RenameActivityRequest,
        api_types::requests::SetCategoryRequest,
        api_types::requests::RegisterRequest,
        api_types::requests::LoginRequest,
        api_types::requests::UpdateProfileRequest,
        api_types::requests::ChangePasswordRequest,
        api_types::requests::CreateReminderRequest,
        api_types::requests::UpdateReminderRequest,
        crate::handlers::auth::RefreshRequest,
        crate::handlers::auth::LogoutRequest,
        api_types::responses::EntryResponse,
        api_types::responses::ActivityResponse,
        api_types::responses::UserResponse,
        api_types::responses::ReminderResponse,
        api_types::responses::DayScheduleResponse,
        api_types::responses::MoodStatsResponse,
        api_types::responses::MoodFrequency,
        api_types::responses::CalendarDayResponse,
        api_types::responses::BulkActionResponse,
        api_types::responses::CorrelationResponse,
        api_types::responses::ImportResultResponse,
        api_types::responses::MediaIdResponse,
        api_types::requests::PushSubscribeRequest,
        api_types::requests::PushUnsubscribeRequest,
    )),
    tags(
        (name = "auth", description = "Authentication"),
        (name = "entries", description = "Mood entries"),
        (name = "activities", description = "Activity catalog"),
        (name = "users", description = "User management"),
        (name = "reminders", description = "Reminder schedules"),
        (name = "media", description = "Photo and voice memo storage"),
        (name = "data", description = "Import and export"),
        (name = "push", description = "Push notifications"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
