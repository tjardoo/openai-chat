use std::error::Error;

pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} [{}] [{}:{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(tracing::log::LevelFilter::Debug)
        // .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()
        .unwrap();

    Ok(())
}
