use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Tracker<'a, T> {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<String>) {
        let count = Rc::strong_count(value);
        let percentage = (count * 100) / self.max;

        if percentage >= 100 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70 {
            self.logger
                .warning(&format!("you have used up over {}% of your quota! Proceeds with precaution", percentage));
        }

        // value field is immutable in this design — it's being tracked externally via Rc count
    }

    pub fn peek(&self, value: &Rc<String>) {
        let percentage = (Rc::strong_count(value) * 100) / self.max;
        self.logger
            .info(&format!("you are using up to {}% of your quota", percentage));
    }
}
