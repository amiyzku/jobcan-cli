use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[clap(about = "Start work")]
    WorkStart(AccountOption),

    #[clap(about = "End work")]
    WorkEnd(AccountOption),

    #[clap(about = "Start rest")]
    RestStart(AccountOption),

    #[clap(about = "End rest")]
    RestEnd(AccountOption),

    #[clap(about = "Working status")]
    Status(AccountOption),
}

#[derive(Debug, Args)]
pub struct AccountOption {
    #[clap(
        short,
        long,
        help = "Account email. Defaults to $JOBCAN_EMAIL if not set.",
        env = "JOBCAN_EMAIL"
    )]
    pub email: Option<String>,

    #[clap(
        short,
        long,
        help = "Account password. Defaults to $JOBCAN_PASSWORD if not set.",
        env = "JOBCAN_PASSWORD"
    )]
    pub password: Option<String>,

    #[clap(
        short,
        long,
        help = "Group ID. Required if you belong to multiple groups. Defaults to $JOBCAN_GROUP_ID if not set.",
        env = "JOBCAN_GROUP_ID"
    )]
    pub group_id: Option<String>,
}
