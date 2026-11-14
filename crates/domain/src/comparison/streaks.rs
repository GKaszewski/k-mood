use std::collections::BTreeSet;

use crate::entry::Date;

pub fn longest_consecutive_run(dates: &BTreeSet<Date>) -> usize {
    let mut longest = 0;
    let mut current = 0;
    let mut previous: Option<Date> = None;
    for date in dates {
        current = match previous {
            Some(before) if before.next() == *date => current + 1,
            _ => 1,
        };
        longest = longest.max(current);
        previous = Some(*date);
    }
    longest
}
