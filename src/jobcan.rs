use std::process::exit;

use anyhow::Result;
use reqwest::Response;

use crate::{
    account::Account, jobcan_html_extractor::JobcanHtmlExtractor, stamp_type::StampType,
    working_status::WorkingStatus,
};

pub struct Jobcan {
    account: Account,
    http_client: reqwest::Client,
}

impl Jobcan {
    const LOGIN_URL: &'static str = "https://id.jobcan.jp/users/sign_in";
    const EMPLOYEE_URL: &'static str = "https://ssl.jobcan.jp/employee";
    const ATTENDANCE_URL: &'static str = "https://ssl.jobcan.jp/employee/attendance";
    const STAMP_URL: &'static str = "https://ssl.jobcan.jp/employee/index/adit";

    pub fn new(account: Account) -> Jobcan {
        Jobcan {
            account,
            http_client: reqwest::Client::builder()
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }

    pub async fn login(&self) -> Result<()> {
        let res = self.fetch_login_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        let html = scraper::Html::parse_document(&body);

        let token = JobcanHtmlExtractor::authenticity_token(&html)?;

        let params = [
            ("authenticity_token", token.as_str()),
            ("user[email]", self.account.email()),
            ("user[password]", self.account.password()),
            ("app_key", "atd"),
            ("commit", "Login"),
        ];

        let res = self
            .http_client
            .post(Self::LOGIN_URL)
            .form(&params)
            .send()
            .await
            .expect("Failed to login request");

        if res.url().path() == "/employee" {
            Ok(())
        } else {
            anyhow::bail!("Login failed");
        }
    }

    pub async fn work_status(&self) -> Result<WorkingStatus> {
        let res = self.fetch_attendance_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        let status = JobcanHtmlExtractor::working_status(&body)?;

        Ok(status)
    }

    pub async fn stamp(
        &self,
        stamp_type: StampType,
        mut group_id: Option<String>,
        is_night_shift: bool,
    ) -> Result<()> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        let html = scraper::Html::parse_document(&body);

        let token = JobcanHtmlExtractor::token(&html)?;

        if group_id.is_none() {
            let group = JobcanHtmlExtractor::group(&html)?;

            if group.len() == 0 {
                anyhow::bail!("Failed to get group");
            } else if group.len() == 1 {
                group_id = Some(group.get(0).unwrap().id().to_string());
            } else {
                group.iter().for_each(|g| {
                    println!("Group name:{}, Group id:{}", g.name(), g.id());
                });
                println!("Please set JOBCAN_GROUP_ID or use `--group-id <GROUP_ID>` option");
                exit(0);
            }
        }

        let is_yakin = if is_night_shift { "1" } else { "0" };

        if let Some(group_id) = group_id {
            let params = [
                ("is_yakin", is_yakin),
                ("adit_item", &stamp_type.to_string()),
                ("notice", ""),
                ("token", token.as_ref()),
                ("adit_group_id", &group_id),
                ("_", ""),
            ];

            let res = self
                .http_client
                .post(Self::STAMP_URL)
                .form(&params)
                .send()
                .await
                .expect("Failed to request work end");

            // TODO: 打刻成功したかどうかの判定を追加する

            return Ok(());
        }

        anyhow::bail!("Unexpected error");
    }

    async fn fetch_login_page(&self) -> Result<Response> {
        let res = self.http_client.get(Self::LOGIN_URL).send().await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                anyhow::bail!("Failed to get login page: {}", e);
            }
        }
    }

    async fn fetch_employee_page(&self) -> Result<Response> {
        let res = self.http_client.get(Self::EMPLOYEE_URL).send().await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                anyhow::bail!("Failed to get employee page: {}", e);
            }
        }
    }

    async fn fetch_attendance_page(&self) -> Result<Response> {
        let res = self.http_client.get(Self::ATTENDANCE_URL).send().await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                anyhow::bail!("Failed to get attendance page: {}", e);
            }
        }
    }
}
