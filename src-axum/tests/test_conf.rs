use secrecy::ExposeSecret;
use serial_test::serial;
use src_axum::configuration::{self, Environment, Settings};



// 加载默认的配置文件
#[serial]
#[tokio::test]
async fn test_config() -> Result<(), anyhow::Error> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("tests/static");
    let Settings {
        application,
        database,
    } = configuration::get_configuration_by_path(configuration_directory).unwrap();
    assert_eq!(application.hmac_secret.expose_secret(), "111111111111111111111111111111111111");
    assert_eq!(application.port, 8000);
    assert_eq!(application.host, "127.0.0.1");
    assert_eq!(database.port, 5432);
    assert_eq!(database.username, "postgres");
    assert_eq!(database.host, "localhost");
    Ok(())
}


// 根据environment决定加载哪个配置文件
#[serial]
#[tokio::test]
async fn test_config_environment() -> Result<(), anyhow::Error> {
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");
    let environment_filename = format!("{}.yaml", environment.as_str());
    assert_eq!(environment_filename, "local.yaml");
    Ok(())

}