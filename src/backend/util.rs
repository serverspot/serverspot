use anyhow::anyhow;
pub fn get_env(name: &str) -> anyhow::Result<String> {
    std::env::var(name).map_err(|_| anyhow!("missing env var {name}"))
}
