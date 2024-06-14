use parking_lot::RwLock;

pub struct Logbox {
    pub logs: RwLock<Vec<Log>>,

    print_log_index: usize
}

pub enum Log{
    Message(String),
    Logbox(Logbox),
}

impl Logbox {
    pub fn new() -> Logbox {
        Logbox {
            logs: RwLock::new(Vec::new()),
            print_log_index: 0
        }
    }

    pub fn push(&mut self, log: Log) {
        self.logs.write().push(log);
    }

    pub fn push_message(&mut self, message: String) {
        self.push(Log::Message(message));
    }

    pub fn push_logbox(&mut self, logbox: Logbox) {
        self.push(Log::Logbox(logbox));
    }

    pub fn print(&mut self) {

        // start iterating from index
        while self.print_log_index < self.logs.read().len() {
            let log = &mut self.logs.write()[self.print_log_index];
            if let Log::Message(message) = log {
                println!("{}", message);
            }else if let Log::Logbox(ref mut logbox) = log {
                logbox.print();
            }
            self.print_log_index += 1;
        }
    }
}