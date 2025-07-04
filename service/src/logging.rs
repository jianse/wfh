use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{daily, hourly, never};
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() -> Option<WorkerGuard> {
    let dirs = settings::get()
        .log
        .level
        .iter()
        .map(|(k, v)| {
            if k == "root" {
                v.to_string()
            } else {
                let mut s = String::from(k);
                s.push('=');
                s.push_str(&v.to_string());
                k.to_owned()
            }
        })
        .fold("".to_owned(), |mut acc, x| {
            acc.push(',');
            acc.push_str(&x);
            acc
        });
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse_lossy(dirs);

    let fmt = fmt::layer()
        .with_ansi(true)
        // .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTime::rfc_3339());

    let lb = tracing_subscriber::registry().with(filter).with(fmt);

    if let Some(file) = settings::get().log.file.clone() {
        let writer = match file.rotate {
            settings::LogRotate::Never => never(file.root, file.prefix),
            settings::LogRotate::Daily => daily(file.root, file.prefix),
            settings::LogRotate::Hourly => hourly(file.root, file.prefix),
        };
        if file.non_blocking {
            let (w, g) = tracing_appender::non_blocking(writer);
            let file_fmt = fmt::layer()
                .with_ansi(false)
                .with_line_number(true)
                .with_timer(LocalTime::rfc_3339())
                .with_writer(w);
            lb.with(file_fmt).try_init().unwrap();

            Some(g)
        } else {
            let file_fmt = fmt::layer()
                .with_ansi(false)
                .with_line_number(true)
                .with_timer(LocalTime::rfc_3339())
                .with_writer(writer);
            lb.with(file_fmt).try_init().unwrap();
            None
        }
    } else {
        lb.try_init().unwrap();
        None
    }
}
