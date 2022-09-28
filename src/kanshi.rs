use crate::{executer::RepeatCacheExecuter};

use std::time::SystemTime;
use std::sync::Mutex;
use std::rc::Rc;

pub struct Kanshi {
    target_file: String,
    updated: Option<SystemTime>,
}
impl Kanshi {
    pub fn new(fname: &str) -> Kanshi {
        Kanshi {
            target_file: String::from(fname),
            updated: None
        }
    }

    pub fn every(&self, seconds: u64) -> RepeatCacheExecuter {
        let am_updated = Rc::new(Mutex::new(self.updated));
        RepeatCacheExecuter::new(&self.target_file, seconds, Rc::clone(&am_updated))
    }
}
