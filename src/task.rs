pub struct Task {
    pub created: f64,
    pub due: f64, 
    pub title: String,
    pub descrition: String,
    pub complete: bool,
}

impl Task {
    pub fn new_blank() -> Self{
        Self {
            created: 0.0,
            due: 0.0,
            title: "".as_string(),
            descrition: "".as_string(),
            complete: false
        }
    }
