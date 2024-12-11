use clap::Args;

#[derive(Args, Debug)]
pub struct NotificationOptions {
    /// API key for the notification service
    #[arg(short, long, env = "KNMI_API_KEY_NOTIFICATION")]
    pub api_key: String,
}
