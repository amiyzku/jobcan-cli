use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub_command: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[clap(about = "Login to Jobcan and auto-detect clock in/out")]
    Auto {
        #[clap(flatten)]
        credentials: Credentials,

        #[clap(flatten)]
        group_id: GroupID,

        #[clap(flatten)]
        night_shift: NightShift,

        #[clap(flatten)]
        note: Notes,
    },

    #[clap(about = "Login to Jobcan and clock in")]
    ClockIn {
        #[clap(flatten)]
        credentials: Credentials,

        #[clap(flatten)]
        group_id: GroupID,

        #[clap(flatten)]
        night_shift: NightShift,

        #[clap(flatten)]
        note: Notes,
    },

    #[clap(about = "Login to Jobcan and clock out")]
    ClockOut {
        #[clap(flatten)]
        credentials: Credentials,

        #[clap(flatten)]
        group_id: GroupID,

        #[clap(flatten)]
        night_shift: NightShift,

        #[clap(flatten)]
        note: Notes,
    },

    #[clap(about = "Login to Jobcan and start break")]
    StartBreak {
        #[clap(flatten)]
        credentials: Credentials,

        #[clap(flatten)]
        group_id: GroupID,

        #[clap(flatten)]
        night_shift: NightShift,

        #[clap(flatten)]
        note: Notes,
    },

    #[clap(about = "Login to Jobcan and end break")]
    EndBreak {
        #[clap(flatten)]
        credentials: Credentials,

        #[clap(flatten)]
        group_id: GroupID,

        #[clap(flatten)]
        night_shift: NightShift,

        #[clap(flatten)]
        note: Notes,
    },

    #[clap(about = "Login to Jobcan and get current working status")]
    Status(Credentials),

    #[clap(about = "Login to Jobcan and list groups which you belong to")]
    ListGroups(Credentials),
}

#[derive(Debug, Args)]
pub struct Credentials {
    #[clap(
        short,
        long,
        help = "Account email. Default to $JOBCAN_EMAIL if not set.",
        env = "JOBCAN_EMAIL"
    )]
    pub email: Option<String>,

    #[clap(
        short,
        long,
        help = "Account password. Default to $JOBCAN_PASSWORD if not set.",
        env = "JOBCAN_PASSWORD"
    )]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct GroupID {
    #[clap(
        long,
        help = "Group ID. Default to $JOBCAN_GROUP_ID if not set.",
        env = "JOBCAN_GROUP_ID"
    )]
    pub group_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct NightShift {
    #[clap(long, default_value = "false", help = "Night-Shift mode.")]
    pub night_shift: bool,
}

impl Into<bool> for NightShift {
    fn into(self) -> bool {
        self.night_shift
    }
}

#[derive(Debug, Args)]
pub struct Notes {
    #[clap(long, default_value = "", help = "Notes to be added to the stamp.")]
    pub notes: String,
}

impl Into<String> for Notes {
    fn into(self) -> String {
        self.notes
    }
}
