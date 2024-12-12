use super::help::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct NotificationOptions {
    #[arg(short, long, env = "KNMI_API_KEY_NOTIFICATION", help = SHORT_HELP_API_KEY_NOTI, long_help = LONG_HELP_API_KEY_NOTI)]
    pub api_key: String,

    #[arg(long, help = SHORT_HELP_DATASET_NAME_NOTI)]
    pub dataset_name: String,

    #[arg(long, help = SHORT_HELP_DATASET_VERSION_NOTI)]
    pub dataset_version: String,

    #[arg(short, long, required = false, env = "KNMI_CLIENT_ID_NOTIFICATION", help = SHORT_HELP_CLIENT_ID_NOTI, long_help = LONG_HELP_CLIENT_ID_NOTI)]
    pub client_id: Option<String>,
}
