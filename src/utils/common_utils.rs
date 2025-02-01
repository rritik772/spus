
#[tracing::instrument(
    name = "check_env"
)]
pub fn check_envs() -> std::io::Result<()> {
    let required_envs = vec![
        "SERVER_HOST",
        "SERVER_PORT",
        "DATABASE_URL"
    ];

    let missing_vars: Vec<_> = required_envs.iter()
        .filter(|v| std::env::var(v).is_err())
        .collect();

    if !missing_vars.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{:?} are missing env variables.", missing_vars)
        ));
    }

    Ok(())
}
