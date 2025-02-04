#![allow(unused)]

use anyhow::Context;

pub fn init_tracing() {
    let logger_init_result  = env_logger::builder().is_test(true).try_init();
    let Ok(_) = logger_init_result else {
        panic!("could not initialize logger: {:?}", logger_init_result.unwrap_err());
    };
}

pub async fn init_postgres(connection_string: &str) -> anyhow::Result<()> {
    let pool = sqlx::PgPool::connect(connection_string)
        .await
        .context("Failed to connect to Postgres")?;
    apply_migrations(&pool).await?;
    Ok(())
}

async fn apply_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    // Rollback all migrations
    // sqlx::query("DELETE FROM _sqlx_migrations WHERE applied = true").execute(pool).await.context("Failed to rollback migrations")?;

    // match sqlx::query("SELECT * FROM migrations WHERE applied = true").execute(pool).await {
    //     Ok(_) => {
    //         println!("Migrations have already been applied.");
    //         return Ok(());
    //     }
    //     Err(e) if e.to_string().contains("no rows in result set") => { }
    //     Err(e) => {
    //         return Err(anyhow::anyhow!("Error checking migrations: {}", e));
    //     }
    // }
    sqlx::migrate!("./supabase/migrations").run(pool).await.context("could not apply migrations")
}
