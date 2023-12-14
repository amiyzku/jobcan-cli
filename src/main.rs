mod account;
mod cli;
mod jobcan;
mod jobcan_html_extractor;
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

    let account_option = match &cli.sub_command {
        cli::SubCommand::WorkStart { account_option, .. }
        | cli::SubCommand::WorkEnd { account_option, .. }
        | cli::SubCommand::RestStart { account_option, .. }
        | cli::SubCommand::RestEnd { account_option, .. }
        | cli::SubCommand::Status(account_option) => account_option,
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

    let group_id = account_option.group_id.as_ref().map(|s| s.to_string());

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
    }

    exit(0);
}
