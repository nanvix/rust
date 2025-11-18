use ::syscall::safe::time::Time;

use crate::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant {
    pub(crate) t: Time,
}

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime {
    pub(crate) t: Time,
}

pub const UNIX_EPOCH: SystemTime = SystemTime { t: Time::EPOCH };

impl Instant {
    pub fn now() -> Instant {
        match Time::now() {
            Ok(time) => Instant { t: time },
            // Fallback to UNIX_EPOCH if the system time cannot be retrieved
            Err(_error) => Instant { t: Time::EPOCH },
        }
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.t.checked_sub(&other.t).ok()
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_add_duration(other)? })
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant { t: self.t.checked_sub_duration(other)? })
    }
}

impl SystemTime {
    pub(crate) fn from_time(time: Time) -> SystemTime {
        SystemTime { t: time }
    }

    pub fn now() -> SystemTime {
        match Time::now() {
            Ok(time) => SystemTime { t: time },
            // Fallback to UNIX_EPOCH if the system time cannot be retrieved
            Err(_error) => SystemTime { t: Time::EPOCH },
        }
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.t.checked_sub(&other.t)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime { t: self.t.checked_add_duration(other)? })
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime { t: self.t.checked_sub_duration(other)? })
    }
}
