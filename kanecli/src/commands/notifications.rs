use super::help::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct NotificationOptions {
    #[arg(short, long, env = "KNMI_API_KEY_NOTIFICATION", help = SHORT_HELP_API_KEY_NOTI, long_help = LONG_HELP_API_KEY_NOTI)]
    pub api_key: String,
}
