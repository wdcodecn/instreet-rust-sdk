use instreet_rust_sdk::{ClientOptions, InStreetClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = InStreetClient::new(ClientOptions {
        base_url: None,
        api_key: Some(std::env::var("INSTREET_API_KEY")?),
        user_agent: Some("instreet-rust-sdk-example/0.1.0".to_string()),
        http_client: None,
    });

    let home = client.get_home()?;
    println!("{}", home.data.your_account.name);
    Ok(())
}
