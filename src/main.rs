mod account;
mod cli;
mod html_extractor;
mod jobcan;
mod stamp_type;
mod working_status;

use std::process::exit;

use account::Account;
use clap::Parser;
use jobcan::Jobcan;
use stamp_type::StampType;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.sub_command {
        cli::SubCommand::ClockIn {
            account_option,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(
                account_option,
                group_id,
                night_shift,
                note,
                StampType::ClockIn,
            )
            .await;
        }
        cli::SubCommand::ClockOut {
            account_option,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(
                account_option,
                group_id,
                night_shift,
                note,
                StampType::ClockOut,
            )
            .await;
        }
        cli::SubCommand::StartBreak {
            account_option,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(
                account_option,
                group_id,
                night_shift,
                note,
                StampType::StartBreak,
            )
            .await;
        }
        cli::SubCommand::EndBreak {
            account_option,
            group_id,
            night_shift,
            note,
            ..
        } => {
            run_stamp(
                account_option,
                group_id,
                night_shift,
                note,
                StampType::EndBreak,
            )
            .await;
        }
        cli::SubCommand::Status(account_option) => {
            run_status(account_option).await;
        }
        cli::SubCommand::ListGroups(account_option) => {
            run_list_groups(account_option).await;
        }
    };

    exit(0);
}

async fn run_stamp(
    account_option: cli::Account,
    group_id: cli::GroupID,
    night_shift: cli::NightShift,
    note: cli::Notes,
    stamp_type: StampType,
) {
    let account = account_from_cli(account_option);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.expect("Failed to login");

    let group_id: String = match group_id.group_id {
        Some(group_id) => group_id,
        None => jobcan
            .default_group_id()
            .await
            .expect("Failed to get default group id"),
    };

    let note: String = note.into();

    jobcan
        .stamp(stamp_type, &group_id, night_shift.into(), &note)
        .await
        .expect("Failed to stamp");
}

async fn run_status(account_option: cli::Account) {
    let account = account_from_cli(account_option);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.expect("Failed to login");

    let status = jobcan
        .work_status()
        .await
        .expect("Failed to get work status");

    println!("{}", status);
}

async fn run_list_groups(account_option: cli::Account) {
    let account = account_from_cli(account_option);
    let jobcan = Jobcan::new(account);

    jobcan.login().await.expect("Failed to login");

    let groups = jobcan
        .list_groups()
        .await
        .expect("Failed to get group list");

    for group in groups {
        println!("GroupID:{}, GroupName:{}", group.id(), group.name());
    }
}

fn account_from_cli(account_option: cli::Account) -> Account {
    match account_option {
        cli::Account {
            email: Some(email),
            password: Some(password),
        } => Account::new(email, password),
        cli::Account {
            email: Some(_),
            password: None,
        } => {
            eprintln!("jobcan password is required.");
            exit(1);
        }
        cli::Account {
            email: None,
            password: Some(_),
        } => {
            eprintln!("jobcan email is required.");
            exit(1);
        }
        cli::Account {
            email: None,
            password: None,
        } => {
            eprintln!("jobcan email is required.");
            eprintln!("jobcan password is required.");
            exit(1);
        }
    }
}
