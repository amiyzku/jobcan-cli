use anyhow::Result;
use reqwest::Response;
use scraper::Html;

use crate::{user::User, working_status::WorkingStatus};

struct Jobcan {
    user: User,
    http_client: reqwest::Client,
}

impl Jobcan {
    const LOGIN_URL: &'static str = "https://id.jobcan.jp/users/sign_in";
    const EMPLOYEE_URL: &'static str = "https://ssl.jobcan.jp/employee";
    const ATTENDANCE_URL: &'static str = "https://ssl.jobcan.jp/employee/attendance";

    pub fn new(user: User) -> Jobcan {
        Jobcan {
            user,
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

        let token = self
            .extract_authenticity_token(html)
            .expect("Failed to get token");

        let params = [
            ("authenticity_token", token.as_str()),
            ("user[email]", self.user.email()),
            ("user[password]", self.user.password()),
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

    pub async fn work_start(&self) {
        todo!()
    }

    pub async fn work_end(&self) {
        todo!()
    }

    pub async fn work_status(&self) -> Result<WorkingStatus> {
        let res = self.fetch_attendance_page().await?;
        let body = res.text().await.expect("Failed to get response body");
        println!("{}", body);
        let status = self.extract_working_status(body)?;

        Ok(status)
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

    fn extract_authenticity_token(&self, html: Html) -> Result<String> {
        let token = html
            .select(&scraper::Selector::parse("input[name=authenticity_token]").unwrap())
            .next()
            .expect("Failed to find authenticity_token")
            .value()
            .attr("value")
            .expect("Failed to get value of authenticity_token")
            .to_string();
        Ok(token)
    }

    fn extract_working_status(&self, text: String) -> Result<WorkingStatus> {
        if text.contains("(勤務中)") {
            Ok(WorkingStatus::Working)
        } else {
            // TODO:他の状態も追加する
            anyhow::bail!("Failed to get working status");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn extract_authenticity_token() {
        // Arrange
        let user = User::new("email", "password");
        let sut = Jobcan::new(user);
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <input name="authenticity_token" value="token">
                </body>
            </html>"""#;
        let html = scraper::Html::parse_document(&body);

        // Act
        let token = sut.extract_authenticity_token(html);

        // Assert
        assert!(token.unwrap() == "token");
    }

    #[tokio::test]
    async fn extract_working_status() {
        // Arrange
        let user = User::new("email", "password");
        let sut = Jobcan::new(user);
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <p>()</p>
                </body>
            </html>"""#;

        // Act
        let status = sut.extract_working_status(body.to_string());

        // Assert
        assert!(status.unwrap() == WorkingStatus::Working);
    }
}
