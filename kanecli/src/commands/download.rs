use super::help::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct DownloadOptions {
    #[arg(long, short, env = "KNMI_API_KEY_OPEN_DATA", help = SHORT_HELP_API_KEY_ODA, long_help = LONG_HELP_API_KEY_ODA)]
    pub api_key: String,

    #[arg(long, short, required = false, default_value = "./output", help = SHORT_HELP_OUTPUT_DIR, long_help = LONG_HELP_OUTPUT_DIR)]
    pub dir: String,

    #[arg(long, short, required = false, help = SHORT_HELP_DOWNLOAD_OUTPUT_FILENAME, long_help = LONG_HELP_DOWNLOAD_OUTPUT_FILENAME)]
    pub filename: Option<String>,

    #[arg(long, short, help = SHORT_HELP_DATASET_NAME)]
    pub name: String,

    #[arg(long, short, help = SHORT_HELP_DATASET_VERSION)]
    pub version: String,
}
