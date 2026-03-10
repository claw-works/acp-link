use anyhow::Result;

/// 入口：加载配置 → 初始化日志 → 启动 LinkService
#[tokio::main]
async fn main() -> Result<()> {
    let config = acp_link::config::AppConfig::discover()?;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::new(&config.log_level))
        .with_ansi(false)
        .init();

    tracing::info!("save_dir: {}", config.storage.save_dir.display());

    let service = acp_link::link::LinkService::new(&config).await?;
    service.run().await
}
