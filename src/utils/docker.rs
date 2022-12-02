use tokio::process::Command;

pub async fn build(path: &str, tag: &str) {
    let command = Command::new("docker build")
        .arg(path)
        .arg(format!("-t {tag}"))
        .output()
        .await;
}
pub async fn start(tag: &str) {
    let command = Command::new("docker run")
        .arg(tag)
        .arg("-it -rm")
        .output()
        .await;
}
