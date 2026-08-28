const DEFAULT_POLL_SECONDS: u64 = 15;
const DEFAULT_SWEEP_SECONDS: u64 = 900;
const DEFAULT_SESSION_CLEANUP_SECONDS: u64 = 3600;
const DEFAULT_REMINDER_SECONDS: u64 = 60;
const DEFAULT_REMINDER_GRACE_MINUTES: i64 = 30;
const DEFAULT_JOBS_PER_POLL: usize = 20;
const DEFAULT_ENQUEUED_PER_SWEEP: usize = 200;
const DEFAULT_MOST_ATTEMPTS: u32 = 5;
const DEFAULT_STALLED_AFTER_SECONDS: i64 = 300;
const DEFAULT_LOOK_UP_WEATHER: bool = true;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(default)]
pub struct WorkerConfig {
    pub poll_seconds: u64,
    pub sweep_seconds: u64,
    pub session_cleanup_seconds: u64,
    pub reminder_seconds: u64,
    pub reminder_grace_minutes: i64,
    pub jobs_per_poll: usize,
    pub enqueued_per_sweep: usize,
    pub most_attempts: u32,
    pub stalled_after_seconds: i64,
    pub look_up_weather: bool,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            poll_seconds: DEFAULT_POLL_SECONDS,
            sweep_seconds: DEFAULT_SWEEP_SECONDS,
            session_cleanup_seconds: DEFAULT_SESSION_CLEANUP_SECONDS,
            reminder_seconds: DEFAULT_REMINDER_SECONDS,
            reminder_grace_minutes: DEFAULT_REMINDER_GRACE_MINUTES,
            jobs_per_poll: DEFAULT_JOBS_PER_POLL,
            enqueued_per_sweep: DEFAULT_ENQUEUED_PER_SWEEP,
            most_attempts: DEFAULT_MOST_ATTEMPTS,
            stalled_after_seconds: DEFAULT_STALLED_AFTER_SECONDS,
            look_up_weather: DEFAULT_LOOK_UP_WEATHER,
        }
    }
}
