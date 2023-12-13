use anyhow::Result;
use scraper::Html;

use crate::working_status::WorkingStatus;

pub struct JobcanHtmlExtractor {}

impl JobcanHtmlExtractor {
    pub fn authenticity_token(html: &Html) -> Result<String> {
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

    pub fn working_status(text: &str) -> Result<WorkingStatus> {
        if text.contains("(勤務中)") {
            Ok(WorkingStatus::Working)
        } else {
            Ok(WorkingStatus::NotWorking)
        }
    }

    pub fn token(html: &Html) -> Result<String> {
        let token = html
            .select(&scraper::Selector::parse("input[name=token]").unwrap())
            .next()
            .expect("Failed to find token")
            .value()
            .attr("value")
            .expect("Failed to get value of token")
            .to_string();
        Ok(token)
    }

    pub fn group_ids(html: &Html) -> Result<Vec<String>> {
        let selector = scraper::Selector::parse("#adit_group_id > option").unwrap();
        let options = html.select(&selector);
        let group_ids = options
            .map(|option| {
                option
                    .value()
                    .attr("value")
                    .expect("Failed to get value of group id")
                    .to_string()
            })
            .collect();
        Ok(group_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_authenticity_token() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <input name="authenticity_token" value="token">
                </body>
            </html>"""#;
        let html = scraper::Html::parse_document(&body);

        // Act
        let token = JobcanHtmlExtractor::authenticity_token(&html);

        // Assert
        assert!(token.unwrap() == "token");
    }

    #[test]
    fn extract_working_status_in_working() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <p>(勤務中)</p>
                </body>
            </html>"""#;

        // Act
        let status = JobcanHtmlExtractor::working_status(body);

        // Assert
        assert!(status.unwrap() == WorkingStatus::Working);
    }

    #[test]
    fn extract_working_status_in_not_working() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <p></p>
                </body>
            </html>"""#;

        // Act
        let status = JobcanHtmlExtractor::working_status(body);

        // Assert
        assert!(status.unwrap() == WorkingStatus::NotWorking);
    }

    #[test]
    fn extract_token() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <input name="token" value="token">
                </body>
            </html>"""#;
        let html = scraper::Html::parse_document(&body);

        // Act
        let token = JobcanHtmlExtractor::token(&html);

        // Assert
        assert!(token.unwrap() == "token");
    }

    #[test]
    fn extract_group_ids_with_multiple_options() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <select id="adit_group_id">
                        <option value="1">1</option>
                        <option value="2">2</option>
                    </select>
                </body>
            </html>"""#;
        let html = scraper::Html::parse_document(&body);

        // Act
        let group_ids = JobcanHtmlExtractor::group_ids(&html);

        // Assert
        assert!(group_ids.unwrap() == vec!["1", "2"]);
    }

    #[test]
    fn extract_group_ids_without_option() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <select id="adit_group_id">
                    </select>
                </body>
            </html>"""#;
        let html = scraper::Html::parse_document(&body);

        // Act
        let group_ids = JobcanHtmlExtractor::group_ids(&html);

        // Assert
        assert!(group_ids.unwrap() == Vec::<String>::new());
    }
}
