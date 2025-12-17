pub enum Status {
    Pass,
    Fail,
    UnderReview,
}

pub struct Blog {
    draft: String,
    content: String,
    status: Status 
}

impl Blog {
    pub fn new(draft: String) -> Blog {
        Blog {
            draft,
            content: "".to_string(),
            status: Status::UnderReview
        }
    }

    pub fn check_status(self) -> Status {
        self.status
    }

    pub fn review(&self) -> &str {
        &self.draft
    }

    pub fn approval(&mut self, check: Status) {
        self.status = check;
        match self.status {
            Status::Pass => self.publish(),
            Status::Fail => {
                self.content = "".to_string();
                println!("content revoked");
            }
            Status::UnderReview => ()
        }
    }

    fn publish(&mut self) {
    self.content = self.draft.clone();
    println!("draft has been published to content");
    }
}
