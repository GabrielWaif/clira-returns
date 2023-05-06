#[derive(serde::Serialize, Debug)]
pub struct AddWorklogDto {
    comment: String,
    started: String,
    timeSpentSeconds: i32
}

impl AddWorklogDto {
    pub fn new(comment: String, started: String, time_spent_seconds: i32) -> AddWorklogDto {
        return AddWorklogDto { comment, started, timeSpentSeconds: time_spent_seconds }
    }
    pub fn set_started(&mut self, started: String) -> () {
        self.started = started;
    }
    pub fn set_comment(&mut self, comment: String) -> () {
        self.comment = comment;
    }
}
