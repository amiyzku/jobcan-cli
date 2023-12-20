mod account;
mod cli;
mod error;
mod html_extractor;
mod jobcan;
mod stamp;
mod working_status;

use std::process::exit;

use account::Account;
use clap::Parser;
use error::JobcanError;
use jobcan::Jobcan;
use stamp::Stamp;

pub type Result<T> = std::result::Result<T, JobcanError>;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.sub_command {
        cli::SubCommand::ClockIn {
            credentials,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(credentials, group_id, night_shift, note, Stamp::ClockIn).await;
        }
        cli::SubCommand::ClockOut {
            credentials,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(credentials, group_id, night_shift, note, Stamp::ClockOut).await;
        }
        cli::SubCommand::StartBreak {
            credentials,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(credentials, group_id, night_shift, note, Stamp::StartBreak).await;
        }
        cli::SubCommand::EndBreak {
            credentials,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(credentials, group_id, night_shift, note, Stamp::EndBreak).await;
        }
        cli::SubCommand::Status(credentials) => {
            run_status(credentials).await;
        }
        cli::SubCommand::ListGroups(credentials) => {
            run_list_groups(credentials).await;
        }
    };

    exit(0);
}

async fn run_stamp(
    credentials: cli::Credentials,
    group_id: cli::GroupID,
    night_shift: cli::NightShift,
    note: cli::Notes,
    stamp_type: Stamp,
) {
    let account = account_from_cli(credentials);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    let group_id: String = match group_id.group_id {
        Some(group_id) => group_id,
        None => jobcan.default_group_id().await.unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        }),
    };

    let note: String = note.into();

    jobcan
        .stamp(stamp_type, &group_id, night_shift.into(), &note)
        .await
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(1);
        });
}

async fn run_status(credentials: cli::Credentials) {
    let account = account_from_cli(credentials);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    let status = jobcan.work_status().await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    println!("{}", status);
}

async fn run_list_groups(credentials: cli::Credentials) {
    let account = account_from_cli(credentials);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    let groups = jobcan.list_groups().await.unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    for group in groups {
        println!("GroupID:{}, GroupName:{}", group.id(), group.name());
    }
}

fn account_from_cli(credentials: cli::Credentials) -> Account {
    match credentials {
        cli::Credentials {
            email: Some(email),
            password: Some(password),
        } => Account::new(email, password),
        cli::Credentials {
            email: Some(_),
            password: None,
        } => {
            eprintln!("jobcan password is required.");
            exit(1);
        }
        cli::Credentials {
            email: None,
            password: Some(_),
        } => {
            eprintln!("jobcan email is required.");
            exit(1);
        }
        cli::Credentials {
            email: None,
            password: None,
        } => {
            eprintln!("jobcan email is required.");
            eprintln!("jobcan password is required.");
            exit(1);
        }
    }
}
