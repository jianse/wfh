use settings::LogFileSettings;
use tracing::Subscriber;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{daily, hourly, never};
use tracing_subscriber::Registry;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::Layered;
use tracing_subscriber::reload::{self, Handle};
use tracing_subscriber::{EnvFilter, Layer, fmt::time::LocalTime};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

type BoxLayer<R> = Box<dyn Layer<R> + Send + Sync + 'static>;

type Reg2 = Layered<BoxLayer<Registry>, Registry>;

type H1 = Handle<EnvFilter, Registry>;

type H2 = Handle<Vec<BoxLayer<Reg2>>, Reg2>;

pub fn bootstrap_logger() -> (H1, H2) {
    let inner = fmt::layer()
        .with_ansi(true)
        .with_line_number(true)
        .with_timer(LocalTime::rfc_3339())
        .boxed();

    let (reload_filter, frh) = reload::Layer::new(EnvFilter::new("info"));
    let (reload_layer, lrh) = reload::Layer::new(vec![inner]);

    // NOTE: we can not reload this two togther
    // See: https://github.com/tokio-rs/tracing/issues/1629
    tracing_subscriber::registry()
        .with(reload_filter.boxed())
        .with(reload_layer)
        .init();
    (frh, lrh)
}

pub fn init_logger() -> Option<WorkerGuard> {
    let (h1, h2) = bootstrap_logger();

    let filter = init_log_level_filter();
    h1.reload(filter).unwrap();

    let (file_layer, guard) = create_file_layer();

    // console logger
    // NOTE: ansi issue in underlying library .
    // See: https://github.com/tokio-rs/tracing/issues/1817
    let console = fmt::layer()
        .with_ansi(true)
        // .with_file(true)
        .with_line_number(true)
        .with_timer(LocalTime::rfc_3339())
        .boxed();

    h2.modify(|layers| {
        let l = layers.pop();
        drop(l);
        if let Some(fl) = file_layer {
            layers.push(fl);
        }
        layers.push(console);
    })
    .unwrap();
    guard
}

fn init_log_level_filter() -> EnvFilter {
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
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .parse(dirs)
        .unwrap()
}

fn create_file_layer<S>() -> (Option<BoxLayer<S>>, Option<WorkerGuard>)
where
    S: Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    if let Some(file) = settings::get().log.file.clone() {
        let (writer, guard) = create_writer(file);
        let file_fmt = fmt::layer()
            .map_fmt_fields(|f| f)
            .with_ansi(false)
            .with_line_number(true)
            .with_timer(LocalTime::rfc_3339())
            .with_writer(writer)
            .boxed();

        (Some(file_fmt), guard)
    } else {
        (None, None)
    }
}

fn create_writer(file: LogFileSettings) -> (BoxMakeWriter, Option<WorkerGuard>) {
    let writer = match file.rotate {
        settings::LogRotate::Never => never(file.root, file.prefix),
        settings::LogRotate::Daily => daily(file.root, file.prefix),
        settings::LogRotate::Hourly => hourly(file.root, file.prefix),
    };
    if file.non_blocking {
        let (w, g) = tracing_appender::non_blocking(writer);
        (BoxMakeWriter::new(w), Some(g))
    } else {
        (BoxMakeWriter::new(writer), None)
    }
}
