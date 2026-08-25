use chrono::{NaiveTime, Weekday};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DaySchedule {
    monday: Option<NaiveTime>,
    tuesday: Option<NaiveTime>,
    wednesday: Option<NaiveTime>,
    thursday: Option<NaiveTime>,
    friday: Option<NaiveTime>,
    saturday: Option<NaiveTime>,
    sunday: Option<NaiveTime>,
}

impl DaySchedule {
    pub fn new() -> Self {
        Self {
            monday: None,
            tuesday: None,
            wednesday: None,
            thursday: None,
            friday: None,
            saturday: None,
            sunday: None,
        }
    }

    pub fn from_persistence(
        monday: Option<NaiveTime>,
        tuesday: Option<NaiveTime>,
        wednesday: Option<NaiveTime>,
        thursday: Option<NaiveTime>,
        friday: Option<NaiveTime>,
        saturday: Option<NaiveTime>,
        sunday: Option<NaiveTime>,
    ) -> Self {
        Self {
            monday,
            tuesday,
            wednesday,
            thursday,
            friday,
            saturday,
            sunday,
        }
    }

    pub fn every_day_at(time: NaiveTime) -> Self {
        Self {
            monday: Some(time),
            tuesday: Some(time),
            wednesday: Some(time),
            thursday: Some(time),
            friday: Some(time),
            saturday: Some(time),
            sunday: Some(time),
        }
    }
}

impl DaySchedule {
    pub fn time_for(&self, day: Weekday) -> Option<NaiveTime> {
        match day {
            Weekday::Mon => self.monday,
            Weekday::Tue => self.tuesday,
            Weekday::Wed => self.wednesday,
            Weekday::Thu => self.thursday,
            Weekday::Fri => self.friday,
            Weekday::Sat => self.saturday,
            Weekday::Sun => self.sunday,
        }
    }

    pub fn has_any_day_set(&self) -> bool {
        self.monday.is_some()
            || self.tuesday.is_some()
            || self.wednesday.is_some()
            || self.thursday.is_some()
            || self.friday.is_some()
            || self.saturday.is_some()
            || self.sunday.is_some()
    }
}

impl DaySchedule {
    pub fn set_time(&mut self, day: Weekday, time: Option<NaiveTime>) {
        match day {
            Weekday::Mon => self.monday = time,
            Weekday::Tue => self.tuesday = time,
            Weekday::Wed => self.wednesday = time,
            Weekday::Thu => self.thursday = time,
            Weekday::Fri => self.friday = time,
            Weekday::Sat => self.saturday = time,
            Weekday::Sun => self.sunday = time,
        }
    }
}

impl Default for DaySchedule {
    fn default() -> Self {
        Self::new()
    }
}
