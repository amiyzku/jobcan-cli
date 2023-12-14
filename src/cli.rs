use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(
        short,
        long,
        help = "Account email. Defaults to JOBCAN_EMAIL if not set.",
        env = "JOBCAN_EMAIL"
    )]
    pub email: Option<String>,

    #[clap(
        short,
        long,
        help = "Account password. Defaults to JOBCAN_PASSWORD if not set.",
        env = "JOBCAN_PASSWORD"
    )]
    pub password: Option<String>,

    #[clap(
        short,
        long,
        help = "Group ID. Required if you belong to multiple groups. Defaults to JOBCAN_GROUP_ID if not set.",
        env = "JOBCAN_GROUP_ID"
    )]
    pub group_id: Option<String>,

    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[clap(about = "Start work")]
    WorkStart,
    #[clap(about = "End work")]
    WorkEnd,
    #[clap(about = "Start rest")]
    RestStart,
    #[clap(about = "End rest")]
    RestEnd,
    #[clap(about = "Working status")]
    Status,
}
