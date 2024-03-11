use alloc::boxed::Box;
use alloc::string::ToString;
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use web_sys::console;
use web_sys::wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct ConsoleLogger {
    level: LevelFilter,
}

impl ConsoleLogger {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn level(self) -> LevelFilter {
        self.level
    }

    #[inline]
    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.level = level;
        self
    }

    /// # Memory leaks
    ///
    /// Calling this function leaks the logger. Please avoid calling it repeateadly.
    #[inline]
    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_max_level(self.level);
        log::set_logger(Box::leak(Box::new(self)))
    }
}

impl Default for ConsoleLogger {
    #[inline]
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
        }
    }
}

impl Log for ConsoleLogger {
    #[inline]
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    #[inline]
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let value = JsValue::from_str(&record.args().to_string());
        match record.level() {
            Level::Error => console::error_1(&value),
            Level::Warn => console::warn_1(&value),
            Level::Info => console::info_1(&value),
            Level::Debug => console::debug_1(&value),
            Level::Trace => console::trace_1(&value),
        }
    }

    #[inline]
    fn flush(&self) {}
}
