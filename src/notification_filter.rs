use core::fmt;

pub struct NotificationFilter {
    pub index: usize,
    pub tag: String,
}

impl fmt::Display for NotificationFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}
