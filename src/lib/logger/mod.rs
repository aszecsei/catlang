mod io;

pub struct Logger;

impl Logger {
    pub fn print(record: &log::Record) {
        io::print(record);
    }

    pub fn set_max_level(level: log::LevelFilter) {
        log::set_max_level(level);
    }
}

impl log::Log for Logger {
    #[inline]
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    #[inline]
    fn log(&self, record: &log::Record) {
        Self::print(record);
    }

    #[inline]
    fn flush(&self) {}
}

#[inline]
pub fn init() -> Result<(), log::SetLoggerError> {
    init_with_max_level(log::LevelFilter::Trace)
}

pub fn init_with_max_level(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    static INSTANCE: Logger = Logger;
    log::set_logger(&INSTANCE).map(|()| log::set_max_level(level))
}
