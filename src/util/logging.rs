pub fn init(is_debug: bool) {
    fern::Dispatch::new()
        .format(move |out, message, _record| {
            if is_debug {
                out.finish(format_args!(
                    "{ts} {msg}",
                    ts = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                    msg = message
                ))
            } else {
                out.finish(format_args!("{msg}", msg = message))
            }
        })
        .level(if is_debug {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .chain(std::io::stdout())
        .apply()
        .expect("logging creation works");
}
