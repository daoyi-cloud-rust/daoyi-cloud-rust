use daoyi_cloud_db::db;
use daoyi_cloud_config::common_config::{self, profile_config::ProfileConfig};
use figment::Figment;
use figment::providers::{Env, Format, Toml};
use tracing::info;

pub async fn init() {
    // 尝试从不同位置获取配置文件路径
    let profile_config_path = format!("{}/resources/config.toml", env!("CARGO_MANIFEST_DIR"));
    let raw_config = Figment::new()
        .merge(Toml::file(&profile_config_path))
        .merge(Env::prefixed("APP_").global());

    let profile_config = match raw_config.extract::<ProfileConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    let config_path = format!(
        "{}/resources/config-{}.toml",
        env!("CARGO_MANIFEST_DIR"),
        profile_config.active
    );

    // 全局初始化
    common_config::init(Some(config_path.clone()));
    let config = common_config::get();

    // 初始化数据库
    db::init(&config.db).await;

    // 初始化日志
    let _guard = config.log.guard();
    info!("log level: {}", &config.log.filter_level);

    info!("app_init success with config {}", config_path);
}
