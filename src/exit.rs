/// Dialog outcome mapped to Zenity-compatible exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Confirmed / closed normally.
    Ok,
    /// Cancelled (Esc, or window closed while a stream was still running).
    Cancel,
    /// Closed automatically by --timeout.
    Timeout,
}

impl Outcome {
    pub fn code(self) -> i32 {
        match self {
            Outcome::Ok => 0,
            Outcome::Cancel => 1,
            Outcome::Timeout => 5,
        }
    }
}
