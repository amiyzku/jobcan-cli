use anyhow::Result;
use reqwest::Response;

use crate::{
    account::Account,
    html_extractor::{Group, HtmlExtractor},
    stamp_type::{self, StampType},
    working_status::WorkingStatus,
};

pub struct Jobcan {
    account: Account,
    http_client: reqwest::Client,
}

impl Jobcan {
    const LOGIN_URL: &'static str = "https://id.jobcan.jp/users/sign_in";
    const EMPLOYEE_URL: &'static str = "https://ssl.jobcan.jp/employee";
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

        let token = HtmlExtractor::authenticity_token(&html)?;

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

    pub async fn stamp(
        &self,
        stamp_type: StampType,
        group_id: &str,
        is_night_shift: bool,
        note: &str,
    ) -> Result<()> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        let html = scraper::Html::parse_document(&body);

        let token = HtmlExtractor::token(&html)?;
        let is_yakin = if is_night_shift { "1" } else { "0" };
        let params = [
            ("is_yakin", is_yakin),
            ("adit_item", &stamp_type.to_request_params()),
            ("notice", note),
            ("token", token.as_ref()),
            ("adit_group_id", group_id),
            ("_", ""),
        ];

        let res = self
            .http_client
            .post(Self::STAMP_URL)
            .form(&params)
            .send()
            .await
            .expect("Failed to request work end");

        if let Some(content_type) = res.headers().get("content-type") {
            if content_type != "application/json" {
                anyhow::bail!("Failed to stamp");
            }

            let json = res
                .json::<stamp_type::Response>()
                .await
                .expect("Failed to parse response to json");

            if json == stamp_type.expected_response() {
                Ok(())
            } else {
                anyhow::bail!("Failed to stamp");
            }
        } else {
            anyhow::bail!("Failed to stamp");
        }
    }

    pub async fn work_status(&self) -> Result<WorkingStatus> {
        let res = self
            .fetch_employee_page()
            .await
            .expect("Failed to get attendance page");
        let body = res.text().await.expect("Failed to get response body");
        let status = HtmlExtractor::working_status(&body).expect("Failed to get working status");

        Ok(status)
    }

    pub async fn list_groups(&self) -> Result<Vec<Group>> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        let html = scraper::Html::parse_document(&body);

        let groups = HtmlExtractor::groups(&html)?;

        Ok(groups)
    }

    pub async fn default_group_id(&self) -> Result<String> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.expect("Failed to get response body");

        let group_id = HtmlExtractor::default_group_id(&body)?;

        Ok(group_id)
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
}
