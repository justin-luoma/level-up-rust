use chrono::{Date, Local};

struct ImportantEvent {
    what: String,
    when: Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::now().date()
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use super::*;

    #[test]
    fn in_past() {
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() - Duration::hours(25),
        };

        assert!(event.is_passed())
    }

    #[test]
    fn in_future() {
        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() + Duration::hours(25),
        };

        assert!(!event.is_passed())
    }
}
