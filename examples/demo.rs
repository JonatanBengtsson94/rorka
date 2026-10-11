use rorka::log_gpu;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    log::info!("demo started");
    let appconfig = rorka::Config {
        title: "Demo".to_string(),
        height: 600,
        width: 800,
    };

    log_gpu();

    if let Err(error) = rorka::run(appconfig) {
        log::error!("failed to run: {error}");
        std::process::exit(1);
    }
}
