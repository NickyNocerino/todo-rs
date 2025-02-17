use std::time::{SystemTime, UNIX_EPOCH};

pub struct Task {
    pub created: f64,
    pub due: f64, 
    pub title: String,
    pub description: String,
    pub complete: bool,
}

impl Task {
    pub fn new_blank() -> Self{
        Self {
            created: 0.0,
            due: 0.0,
            title: "".to_string(),
            description: "".to_string(),
            complete: false
        }
    }

    pub fn new_entry(due: f64, title:String, description: String) -> Self{
        let now = SystemTime::now();
        let since_the_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
        Self {
            created: since_the_epoch.as_secs_f64(),
            due: due,
            title: title,
            description:description,
            complete: false
        }
    }

    pub fn new_backdated_entry(created: f64, due: f64, title:String, description: String) -> Self{
        Self {
            created: created,
            due: due,
            title: title,
            description:description,
            complete: false
        }
    }

    pub fn is_between(&self, start_time: f64, end_time: f64) -> bool {
        if {
            self.due > start_time
            &&
            self.due < end_time
        }
        {
            true
        }
        false
    }
}


