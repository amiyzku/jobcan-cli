use reqwest::Response;

use crate::{
    account::Account,
    error::JobcanError,
    html_extractor::{Group, HtmlExtractor},
    stamp::{self, Stamp},
    working_status::WorkingStatus,
    Result,
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
        let body = res.text().await.map_err(|e| JobcanError::ReqwestError {
            message: "Failed to get contents in login page".into(),
            url: Self::LOGIN_URL.into(),
            raw_error: e,
        })?;
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
            .map_err(|e| JobcanError::ReqwestError {
                message: "Failed to request login".into(),
                url: Self::LOGIN_URL.into(),
                raw_error: e,
            })?;

        if res.url().path() == "/employee" {
            Ok(())
        } else {
            Err(JobcanError::AuthError.into())
        }
    }

    pub async fn stamp(
        &self,
        stamp_type: Stamp,
        group_id: &str,
        is_night_shift: bool,
        note: &str,
    ) -> Result<()> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.map_err(|e| JobcanError::ReqwestError {
            message: "Failed to get contents in employee page".into(),
            url: Self::EMPLOYEE_URL.into(),
            raw_error: e,
        })?;
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
            .map_err(|e| JobcanError::ReqwestError {
                message: format!("Failed to request {}", stamp_type),
                url: Self::STAMP_URL.into(),
                raw_error: e,
            })?;

        self.handle_stamp_response(res, stamp_type).await
    }

    pub async fn work_status(&self) -> Result<WorkingStatus> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.map_err(|e| JobcanError::ReqwestError {
            message: "Failed to get contents in employee page".into(),
            url: Self::EMPLOYEE_URL.into(),
            raw_error: e,
        })?;
        HtmlExtractor::working_status(&body)
    }

    pub async fn list_groups(&self) -> Result<Vec<Group>> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.map_err(|e| JobcanError::ReqwestError {
            message: "Failed to get contents in employee page".into(),
            url: Self::EMPLOYEE_URL.into(),
            raw_error: e,
        })?;
        let html = scraper::Html::parse_document(&body);
        HtmlExtractor::groups(&html)
    }

    pub async fn default_group_id(&self) -> Result<String> {
        let res = self.fetch_employee_page().await?;
        let body = res.text().await.map_err(|e| JobcanError::ReqwestError {
            message: "Failed to get contents in employee page".into(),
            url: Self::EMPLOYEE_URL.into(),
            raw_error: e,
        })?;
        HtmlExtractor::default_group_id(&body)
    }

    async fn fetch_login_page(&self) -> Result<Response> {
        self.http_client
            .get(Self::LOGIN_URL)
            .send()
            .await
            .map_err(|e| JobcanError::ReqwestError {
                message: "Failed to request login page".into(),
                url: Self::LOGIN_URL.into(),
                raw_error: e,
            })
    }

    async fn fetch_employee_page(&self) -> Result<Response> {
        self.http_client
            .get(Self::EMPLOYEE_URL)
            .send()
            .await
            .map_err(|e| JobcanError::ReqwestError {
                message: "Failed to request employee page".into(),
                url: Self::EMPLOYEE_URL.into(),
                raw_error: e,
            })
    }

    async fn handle_stamp_response(&self, res: Response, stamp_type: Stamp) -> Result<()> {
        let content_type = res.headers().get("content-type").expect("No content-type");
        if content_type != "application/json" {
            return Err(JobcanError::UnexpectedResponseError {
                message: format!(
                    "Unexpected content-type found: expected application/json, got `{:?}`",
                    content_type
                ),
            });
        }

        let json = res
            .json::<stamp::Response>()
            .await
            .map_err(|e| JobcanError::ReqwestError {
                message: "Failed to parse response".into(),
                url: Self::STAMP_URL.into(),
                raw_error: e,
            })?;

        if json == stamp_type.expected_response() {
            Ok(())
        } else {
            Err(JobcanError::UnexpectedResponseError {
                message: format!(
                    "Unexpected response found: expected `{:?}`, got `{:?}`",
                    stamp_type.expected_response(),
                    json
                ),
            })
        }
    }
}
