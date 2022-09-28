use std::io::Write;
use std::fs::OpenOptions;
use std::sync::Mutex;
use std::ffi::CStr;
use std::path::Path;

//usage: 
//  let logger = Logger::new("/root/myplayground/test.log");
//  logger.write(LogLevel::Info, "(^ _ ^)");
//  logger.write(LogLevel::Warn, "(- _ -)");

pub enum LogLevel {
    Info,
    Warn,
}

pub struct Logger<'a> {
    logfile: &'a str,
}
impl Logger<'_> {
    pub fn new(logfile: &str) -> Logger {
        OpenOptions::new().create(true).write(true).open(&Path::new(logfile)).unwrap();
        Logger{
            logfile
        }
    }

    /**
     * - 時間
     * - Loglevel
     * - logシステム名([kanshi]的な)
     * - ログメッセージ
     */
    pub fn write(&self, level: LogLevel, msg: &str) -> () {
        // get current time
        let mut curtime: i64 = 0 as i32 as i64;
        unsafe{time(&mut curtime)};
        let current: *mut i8 = unsafe{ctime(&mut curtime)};
        let current_str: &CStr = unsafe { CStr::from_ptr(current) };
        let current_time: &str = current_str.to_str().unwrap().trim();
        let log = Mutex::new(String::new());
        OpenOptions::new()
            .append(true)
            .open(self.logfile)
            .expect("file open options failed something..")
            .write_all({
                match level {
                    LogLevel::Info => {
                        let mut log = log.lock().unwrap();
                        let log_content = format!("[INFO] {}\n", msg);
                        *log = log_content;
                    },
                    LogLevel::Warn => {
                        let mut log = log.lock().unwrap();
                        let log_content = format!("[WARN] {}\n", msg);
                        *log = log_content;
                    },
                }
                //log.lock().unwrap().as_bytes()
                format!("{} Kanshi{}", current_time, log.lock().unwrap()).as_bytes()
            })
            .expect("something Errorr");
    }
}

extern "C" {
    fn time(timer: *mut i64) -> i64;
    fn ctime(timer: *const i64) -> *mut i8;
}
