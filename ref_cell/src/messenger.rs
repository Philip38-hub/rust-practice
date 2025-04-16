use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Tracker { logger, max }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let percentage = count * 100 / self.max;

        if percentage >= 100 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70 {
            self.logger.warning(&format!(
                "Warning: you have used up over {}% of your quota! Proceeds with precaution",
                percentage
            ));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        let percentage = count * 100 / self.max;
        self.logger.info(&format!(
            "Info: you are using up to {}% of your quota",
            percentage
        ));
    }
}
