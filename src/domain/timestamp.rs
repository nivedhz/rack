pub struct TimeStamp(i64);

impl TimeStamp {
    pub fn now() -> TimeStamp {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            Ok(millis) => TimeStamp(millis.as_millis() as i64),
            Err(e) => panic!("SystemTime before UNIX_EPOCH! {}", e),
        }
    }
    pub fn from_millis(millis: i64) -> TimeStamp {
        TimeStamp(millis)
    }
    pub fn as_millis(self) -> i64 {
        self.0
    }
}

impl std::fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
