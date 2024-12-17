use super::help::*;
use clap::Args;

#[derive(Args, Debug)]
pub struct DownloadOptions {
    #[arg(long, env = "KNMI_API_KEY_OPEN_DATA", help = SHORT_HELP_API_KEY_ODA, long_help = LONG_HELP_API_KEY_ODA)]
    pub api_key: String,

    #[arg(long, required = false, default_value = "./output", help = SHORT_HELP_OUTPUT_DIR, long_help = LONG_HELP_OUTPUT_DIR)]
    pub output_dir: String,

    #[arg(long, required = false, help = SHORT_HELP_DOWNLOAD_OUTPUT_FILENAME, long_help = LONG_HELP_DOWNLOAD_OUTPUT_FILENAME)]
    pub output_filename: Option<String>,

    #[arg(long, help = SHORT_HELP_DATASET_NAME)]
    pub dataset_name: String,

    #[arg(long, help = SHORT_HELP_DATASET_VERSION)]
    pub dataset_version: String,
}
