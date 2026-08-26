use domain::weather::Condition;

pub fn condition_for(code: u8) -> Option<Condition> {
    match code {
        0..=1 => Some(Condition::Clear),
        2..=3 => Some(Condition::Cloudy),
        45..=48 => Some(Condition::Fog),
        51..=57 => Some(Condition::Drizzle),
        61..=67 | 80..=82 => Some(Condition::Rain),
        71..=77 | 85..=86 => Some(Condition::Snow),
        95..=99 => Some(Condition::Thunderstorm),
        _ => None,
    }
}
