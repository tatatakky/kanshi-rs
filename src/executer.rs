use std::thread;
use std::fs;
use std::time::{Duration, SystemTime};
use std::sync::Mutex;
use std::rc::Rc;

use crate::log::{Logger, LogLevel};

pub struct RepeatCacheExecuter<'a> {
    target_file: &'a String,
    every: u64,
    am_updated: Rc<Mutex<Option<SystemTime>>>,
}
impl RepeatCacheExecuter<'_> {
    pub fn new(target_file: &String, every: u64, am_updated: Rc<Mutex<Option<SystemTime>>>) -> RepeatCacheExecuter {
        RepeatCacheExecuter {
            target_file,
            every,
            am_updated
        }
    }
    pub fn lazy_exec<T>(&mut self, mut closure: T) where T: FnMut() -> () {
        loop {
            let meta = fs::metadata(self.target_file).unwrap();
            let now_timestamp = meta.modified().expect("Error");
            let mut opt_cached_time = self.am_updated.lock().unwrap();
            match *opt_cached_time {
                Some(cached_timestamp) => {
                    if cached_timestamp.ne(&now_timestamp) {
                        *opt_cached_time= Some(now_timestamp);
                        let logger = Logger::new("/var/log/kanshi.log");
                        logger.write(LogLevel::Info, "File Timestamp is changed now, then execute script!");
                        closure();
                    }
                },
                None => {
                    *opt_cached_time = Some(now_timestamp);
                    let logger = Logger::new("/var/log/kanshi.log");
                    logger.write(LogLevel::Info, "Start to do monitoring File Timestamp!");
                    println!("start monitoring!");
                }
            }
            thread::sleep(Duration::from_secs(self.every));
        }
    }
}
