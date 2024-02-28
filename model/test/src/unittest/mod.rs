pub mod setup {
    pub mod log {
        use std::sync::Once;

        static SETUP: Once = Once::new();
        pub fn configure() {
            configure_level(log::LevelFilter::Trace)
        }
        pub fn configure_level(level: log::LevelFilter) {
            configure_level_internal(level, false)
        }
        pub fn configure_compact(level: log::LevelFilter) {
            configure_level_internal(level, true)
        }
        fn configure_level_internal(level: log::LevelFilter, compact: bool) {
            SETUP.call_once(|| {
                use colored::*;
                use std::io::Write;
                if !compact {
                    let _ = env_logger::builder()
                        .filter_level(level)
                        .format(|buf, record| {
                            static mut MAX_THREAD_WITH: usize = 20;
                            let ts = buf.timestamp_nanos();
                            let level = match record.level() {
                                log::Level::Error => "ERROR".red(),
                                log::Level::Warn => "WARN ".yellow(),
                                log::Level::Info => "INFO ".green(),
                                log::Level::Debug => "DEBUG".blue(),
                                log::Level::Trace => "TRACE".blue(),
                            };
                            let target = record.target();
                            let args = record.args();

                            let thread = std::thread::current();
                            let id = thread.id();
                            let name = thread.name().unwrap_or(format!("Thread-{id:?}").as_str()).to_owned();
                            unsafe { MAX_THREAD_WITH = MAX_THREAD_WITH.max(name.len()) };
                            let name = format!("{: <0width$}", name, width = unsafe { MAX_THREAD_WITH });
                            writeln!(buf, "{ts} {level} ({name}) {target} {args}")
                        })
                        // .format_timestamp_micro s()
                        .is_test(false) // disables color in the terminal
                        .try_init();
                } else {
                    let _ = env_logger::builder()
                        .filter_level(level)
                        .format(|buf, record| {
                            let ts = buf.timestamp_nanos();
                            let level = match record.level() {
                                log::Level::Error => "ERROR".red(),
                                log::Level::Warn => "WARN ".yellow(),
                                log::Level::Info => "INFO ".green(),
                                log::Level::Debug => "DEBUG".blue(),
                                log::Level::Trace => "TRACE".blue(),
                            };
                            let pkg_name = record.target().to_owned();
                            let split = pkg_name.split("::").map(|x| x.to_owned()).collect::<Vec<_>>();
                            let first = split.first().unwrap();
                            let mut it = split.iter().rev();
                            let _ = it.next();
                            let last = it.next().map_or("".to_owned(), |x| x.to_owned());
                            let args = record.args();

                            let thread = std::thread::current();
                            let id = thread.id();
                            let mut name = thread.name().unwrap_or(format!("Thread-{id:?}").as_str()).to_owned();
                            if name.contains("::") {
                                name = "main-Thread".to_owned();
                            }

                            writeln!(buf, "{ts} {level} ({name}) {first}::*::{last} {args}")
                        })
                        // .format_timestamp_micro s()
                        .is_test(false) // disables color in the terminal
                        .try_init();
                }
            });
        }
    }
}
