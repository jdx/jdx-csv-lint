#[ctor::ctor]
fn init() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_env("JDX_CSV_LINT_LOG_LEVEL")
        .format_timestamp(None).init();
}
