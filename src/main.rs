mod datasources;
mod error;
mod provider;
mod resources;

#[snafu::report]
#[tokio::main]
async fn main() -> Result<(), snafu::Whatever> {
    tf_provider::serve("nixos", provider::NixosProvider::default())
        .await
        .whatever_context("failed to serve provider")?;

    Ok(())
}
