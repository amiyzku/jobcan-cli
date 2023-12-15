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

    let (account_option, group_id) = match &cli.sub_command {
        cli::SubCommand::WorkStart {
            account_option,
            group_id,
            ..
        }
        | cli::SubCommand::WorkEnd {
            account_option,
            group_id,
            ..
        }
        | cli::SubCommand::RestStart {
            account_option,
            group_id,
            ..
        }
        | cli::SubCommand::RestEnd {
            account_option,
            group_id,
            ..
        } => (account_option, Some(group_id)),
        cli::SubCommand::Status(account_option) | cli::SubCommand::ListGroups(account_option) => {
            (account_option, None)
        }
    };

    if account_option.email.is_none() {
        eprintln!("jobcan email is required.");
        exit(1);
    }

    if account_option.password.is_none() {
        eprintln!("jobcan password is required.");
        exit(1);
    }

    let account = Account::new(
        account_option.email.as_ref().unwrap().to_string(),
        account_option.password.as_ref().unwrap().to_string(),
    );
    let jobcan = Jobcan::new(account);

    jobcan.login().await.unwrap();

    let group_id = group_id.map(|s| s.group_id.as_ref().unwrap().clone());

    match cli.sub_command {
        cli::SubCommand::WorkStart { night_shift, .. } => {
            jobcan
                .stamp(StampType::WorkStart, group_id, night_shift.into())
                .await
                .unwrap();
        }
        cli::SubCommand::WorkEnd { night_shift, .. } => {
            jobcan
                .stamp(StampType::WorkEnd, group_id, night_shift.into())
                .await
                .unwrap();
        }
        cli::SubCommand::RestStart { night_shift, .. } => {
            jobcan
                .stamp(StampType::RestStart, group_id, night_shift.into())
                .await
                .unwrap();
        }
        cli::SubCommand::RestEnd { night_shift, .. } => {
            jobcan
                .stamp(StampType::RestEnd, group_id, night_shift.into())
                .await
                .unwrap();
        }
        cli::SubCommand::Status(_) => {
            let status = jobcan.work_status().await.unwrap();
            println!("{}", status);
        }
        cli::SubCommand::ListGroups(_) => {
            let groups = jobcan.list_groups().await.unwrap();
            for group in groups {
                println!("{}: {}", group.id(), group.name());
            }
        }
    }

    exit(0);
}
