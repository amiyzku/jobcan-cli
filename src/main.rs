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

    if cli.email.is_none() {
        eprintln!("JOBCAN_EMAIL must be set.");
        exit(1);
    }

    if cli.password.is_none() {
        eprintln!("JOBCAN_PASSWORD must be set.");
        exit(1);
    }

    let account = Account::new(cli.email.unwrap(), cli.password.unwrap());
    let jobcan = Jobcan::new(account);

    jobcan.login().await.unwrap();

    match cli.sub_command {
        cli::SubCommand::WorkStart => {
            jobcan
                .stamp(StampType::WorkStart, cli.group_id)
                .await
                .unwrap();
        }
        cli::SubCommand::WorkEnd => {
            jobcan
                .stamp(StampType::WorkEnd, cli.group_id)
                .await
                .unwrap();
        }
        cli::SubCommand::RestStart => {
            jobcan
                .stamp(StampType::RestStart, cli.group_id)
                .await
                .unwrap();
        }
        cli::SubCommand::RestEnd => {
            jobcan
                .stamp(StampType::RestEnd, cli.group_id)
                .await
                .unwrap();
        }
        cli::SubCommand::Status => {
            let status = jobcan.work_status().await.unwrap();
            println!("{}", status);
        }
    }

    exit(0);
}
