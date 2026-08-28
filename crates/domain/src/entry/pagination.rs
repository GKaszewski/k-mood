use crate::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pagination {
    limit: i64,
    offset: i64,
}

impl Pagination {
    pub fn new(limit: i64, offset: i64, most_per_page: i64) -> Result<Self, DomainError> {
        if limit < 1 {
            return Err(DomainError::InvalidInput(
                "a page of nothing is not a page: ask for at least one".into(),
            ));
        }

        if limit > most_per_page {
            return Err(DomainError::InvalidInput(format!(
                "this server serves at most {most_per_page} entries per page, and {limit} were asked for"
            )));
        }

        if offset < 0 {
            return Err(DomainError::InvalidInput(
                "a page cannot start before the beginning".into(),
            ));
        }

        Ok(Self { limit, offset })
    }

    pub fn limit(&self) -> i64 {
        self.limit
    }

    pub fn offset(&self) -> i64 {
        self.offset
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    items: Vec<T>,
    total: u64,
    at: Pagination,
}

impl<T> Page<T> {
    pub fn new(items: Vec<T>, total: u64, at: Pagination) -> Self {
        Self { items, total, at }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    pub fn total(&self) -> u64 {
        self.total
    }

    pub fn limit(&self) -> i64 {
        self.at.limit()
    }

    pub fn offset(&self) -> i64 {
        self.at.offset()
    }

    pub fn more_after_this(&self) -> bool {
        let seen = self.at.offset().saturating_add(self.items.len() as i64);

        (seen as u64) < self.total
    }

    pub fn map<U>(self, transform: impl FnMut(T) -> U) -> Page<U> {
        Page {
            items: self.items.into_iter().map(transform).collect(),
            total: self.total,
            at: self.at,
        }
    }
}
