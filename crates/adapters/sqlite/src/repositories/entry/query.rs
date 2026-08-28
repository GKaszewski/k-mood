use sqlx::SqlitePool;

use domain::activity::ActivityId;
use domain::entry::{DateRange, EntrySelection, MoodEntry, MoodEntryId, Pagination};
use domain::errors::DomainError;
use domain::user::UserId;

use super::super::shared::{db_err, sortable_instant};
use super::rows::{EntryRow, hydrate_batch, hydrate_single};

pub struct SqliteEntryQueryRepository {
    pool: SqlitePool,
}

impl SqliteEntryQueryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl domain::ports::MoodEntryQueryPort for SqliteEntryQueryRepository {
    async fn find_by_id(&self, id: &MoodEntryId) -> Result<Option<MoodEntry>, DomainError> {
        let row = sqlx::query_as::<_, EntryRow>("SELECT * FROM mood_entries WHERE id = ?")
            .bind(id.value().to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(db_err)?;

        match row {
            Some(r) => Ok(Some(hydrate_single(&self.pool, r).await?)),
            None => Ok(None),
        }
    }

    async fn find_all_by_user(&self, user_id: &UserId) -> Result<Vec<MoodEntry>, DomainError> {
        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? ORDER BY logged_at DESC",
        )
        .bind(user_id.value().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn select(
        &self,
        selection: &EntrySelection,
        page: Pagination,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let narrowing = Narrowing::of(selection);

        let sql = format!(
            "SELECT me.* FROM mood_entries me{} WHERE {} ORDER BY me.logged_at DESC LIMIT ? OFFSET ?",
            narrowing.join, narrowing.conditions
        );

        let mut query = sqlx::query_as::<_, EntryRow>(sqlx::AssertSqlSafe(sql));
        query = narrowing.bind(query);

        let rows = query
            .bind(page.limit())
            .bind(page.offset())
            .fetch_all(&self.pool)
            .await
            .map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn count(&self, selection: &EntrySelection) -> Result<u64, DomainError> {
        let narrowing = Narrowing::of(selection);

        let sql = format!(
            "SELECT COUNT(*) FROM mood_entries me{} WHERE {}",
            narrowing.join, narrowing.conditions
        );

        let mut query = sqlx::query_as::<_, (i64,)>(sqlx::AssertSqlSafe(sql));
        query = narrowing.bind(query);

        let (counted,) = query.fetch_one(&self.pool).await.map_err(db_err)?;

        Ok(u64::try_from(counted).unwrap_or(0))
    }

    async fn find_by_date_range(
        &self,
        user_id: &UserId,
        range: &DateRange,
    ) -> Result<Vec<MoodEntry>, DomainError> {
        let rows = sqlx::query_as::<_, EntryRow>(
            "SELECT * FROM mood_entries WHERE user_id = ? AND logged_at >= ? AND logged_at <= ? ORDER BY logged_at DESC",
        )
        .bind(user_id.value().to_string())
        .bind(sortable_instant(range.start()))
        .bind(sortable_instant(range.end()))
        .fetch_all(&self.pool).await.map_err(db_err)?;

        hydrate_batch(&self.pool, rows).await
    }

    async fn count_tagged_with(&self, activity_id: &ActivityId) -> Result<u64, DomainError> {
        let (count,): (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM entry_activities WHERE activity_id = ?")
                .bind(activity_id.value().to_string())
                .fetch_one(&self.pool)
                .await
                .map_err(db_err)?;

        Ok(u64::try_from(count).unwrap_or(0))
    }
}

struct Narrowing<'a> {
    join: &'static str,
    conditions: String,
    selection: &'a EntrySelection,
}

impl<'a> Narrowing<'a> {
    fn of(selection: &'a EntrySelection) -> Self {
        let mut conditions = vec!["me.user_id = ?".to_string()];

        if selection.range().is_some() {
            conditions.push("me.logged_at >= ?".into());
            conditions.push("me.logged_at <= ?".into());
        }
        if selection.mood().is_some() {
            conditions.push("me.mood = ?".into());
        }
        if selection.activity().is_some() {
            conditions.push("ea.activity_id = ?".into());
        }
        if selection.since().is_some() {
            conditions.push("me.updated_at > ?".into());
        }

        Self {
            join: match selection.activity() {
                Some(_) => " INNER JOIN entry_activities ea ON ea.entry_id = me.id",
                None => "",
            },
            conditions: conditions.join(" AND "),
            selection,
        }
    }

    fn bind<T>(
        &self,
        mut query: sqlx::query::QueryAs<'a, sqlx::Sqlite, T, sqlx::sqlite::SqliteArguments>,
    ) -> sqlx::query::QueryAs<'a, sqlx::Sqlite, T, sqlx::sqlite::SqliteArguments> {
        query = query.bind(self.selection.user_id().value().to_string());

        if let Some(range) = self.selection.range() {
            query = query.bind(sortable_instant(range.start()));
            query = query.bind(sortable_instant(range.end()));
        }
        if let Some(mood) = self.selection.mood() {
            query = query.bind(i32::from(mood.value()));
        }
        if let Some(activity_id) = self.selection.activity() {
            query = query.bind(activity_id.value().to_string());
        }
        if let Some(since) = self.selection.since() {
            query = query.bind(since.to_rfc3339());
        }

        query
    }
}
