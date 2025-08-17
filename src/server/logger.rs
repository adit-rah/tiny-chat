use tokio::fs::File;
use tokio::fs::OpenOptions;

pub type LogFile = File;

pub async fn init_log_file(path: &str) -> LogFile {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .await
        .unwrap()
}
