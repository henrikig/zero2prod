use zero2prod::{
    configuration::get_configuration,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration file.");

    let server = startup::build(configuration).await?;
    server.await?;
    Ok(())
}
