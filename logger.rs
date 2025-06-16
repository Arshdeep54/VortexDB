use flexi_logger::{ DeferredNow, LogSpecification, Logger };
use flexi_logger::writers::LogWriter;
use log::Record;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::{ create_dir_all, OpenOptions };
use std::io::{ self, Write };
use std::path::Path;
use std::sync::{ Mutex, Once };

// Cache for writers based on log target name
static MODULE_WRITERS: Lazy<Mutex<HashMap<String, Box<dyn Write + Send>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

// Thread-local to store per-thread log file name
thread_local! {
    static CURRENT_LOG_TARGET: RefCell<Option<String>> = RefCell::new(None);
}

static INIT: Once = Once::new();

/// Call this once per file (module) to set up logging for that file.
/// Logs will go to logs/<name>.log
pub fn init_logger_with_name(name: &str) {
    // Store the log target in thread-local storage
    CURRENT_LOG_TARGET.with(|slot| {
        *slot.borrow_mut() = Some(name.to_string());
    });

    // Only start the logger once globally
    INIT.call_once(|| {
        Logger::with(LogSpecification::parse("debug").unwrap())
            .log_to_writer(Box::new(NamedModuleWriter {}))
            .format(custom_format)
            .start()
            .unwrap();
    });
}

// The custom log format (shared for all writers)
fn custom_format(w: &mut dyn Write, now: &mut DeferredNow, record: &Record) -> io::Result<()> {
    write!(
        w,
        "[{}][{}][{}:{}] {}\n",
        now.format("%Y-%m-%d %H:%M:%S"),
        record.level(),
        record.module_path().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        &record.args()
    )
}

// LogWriter that writes to the file specified by CURRENT_LOG_TARGET
pub struct NamedModuleWriter;

impl LogWriter for NamedModuleWriter {
    fn write(&self, now: &mut DeferredNow, record: &Record) -> io::Result<()> {
        let name = CURRENT_LOG_TARGET.with(|slot| {
            slot.borrow()
                .clone()
                .unwrap_or_else(|| "unknown".to_string())
        });

        let mut map = MODULE_WRITERS.lock().unwrap();

        let writer = map.entry(name.clone()).or_insert_with(|| {
            let log_dir = Path::new("logs");
            create_dir_all(log_dir).unwrap();

            let file_path = log_dir.join(format!("{}.log", name));
            let file = OpenOptions::new().create(true).append(true).open(file_path).unwrap();

            Box::new(file) as Box<dyn Write + Send>
        });

        custom_format(&mut **writer, now, record)
    }

    fn flush(&self) -> io::Result<()> {
        for writer in MODULE_WRITERS.lock().unwrap().values_mut() {
            writer.flush()?;
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! init_module_logger {
    ($name:expr) => {
        crate::logger::init_logger_with_name($name)
    };
}
