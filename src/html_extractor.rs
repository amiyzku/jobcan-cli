use regex::Regex;
use scraper::Html;

use crate::{error::JobcanError, working_status::WorkingStatus};

#[derive(Debug, PartialEq, Eq)]
pub struct Group {
    id: String,
    name: String,
}

impl Group {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct HtmlExtractor {}

impl HtmlExtractor {
    pub fn authenticity_token(html: &Html) -> Result<String, JobcanError> {
        let token = html
            .select(&scraper::Selector::parse("input[name=authenticity_token]").unwrap())
            .next()
            .ok_or_else(|| JobcanError::ElementExtractError {
                message: "Failed to find `input[name=authenticity_token]`".into(),
            })?
            .value()
            .attr("value")
            .ok_or_else(|| JobcanError::ElementExtractError {
                message: "Failed to get value of `input[name=authenticity_token]`".into(),
            })?
            .to_string();
        Ok(token)
    }

    pub fn working_status(text: &str) -> Result<WorkingStatus, JobcanError> {
        let re = Regex::new(r#"var current_status = "(.*?)";"#).unwrap();
        match re.captures(text) {
            Some(caps) => match caps.get(1).unwrap().as_str() {
                "returned_home" | "having_breakfast" => Ok(WorkingStatus::NotWorking),
                "working" => Ok(WorkingStatus::Working),
                "resting" => Ok(WorkingStatus::Resting),
                _ => Err(JobcanError::UnexpectedResponseError {
                    message: "Failed to get working status".into(),
                }),
            },
            None => Err(JobcanError::UnexpectedResponseError {
                message: "Failed to get working status".into(),
            }),
        }
    }

    pub fn token(html: &Html) -> Result<String, JobcanError> {
        let token = html
            .select(&scraper::Selector::parse("input[name=token]").unwrap())
            .next()
            .ok_or_else(|| JobcanError::ElementExtractError {
                message: "Failed to find `input[name=token]`".into(),
            })?
            .value()
            .attr("value")
            .ok_or_else(|| JobcanError::ElementExtractError {
                message: "Failed to get value of `input[name=token]`".into(),
            })?
            .to_string();
        Ok(token)
    }

    pub fn groups(html: &Html) -> Result<Vec<Group>, JobcanError> {
        let selector = scraper::Selector::parse("#adit_group_id > option").unwrap();
        let options = html.select(&selector);

        let mut group_ids = Vec::new();
        for option in options {
            let id = option
                .value()
                .attr("value")
                .ok_or_else(|| JobcanError::ElementExtractError {
                    message: "Failed to get value of group id".into(),
                })?
                .to_string();
            let name = option
                .text()
                .next()
                .ok_or_else(|| JobcanError::ElementExtractError {
                    message: "Failed to get value of group name".into(),
                })?
                .to_string();
            group_ids.push(Group { id, name });
        }

        Ok(group_ids)
    }

    pub fn default_group_id(text: &str) -> Result<String, JobcanError> {
        let re = Regex::new(r#"var defaultAditGroupId = (.*?);"#).unwrap();
        match re.captures(text) {
            Some(caps) => match caps.get(1) {
                Some(group_id) => Ok(group_id.as_str().to_string()),
                None => Err(JobcanError::UnexpectedResponseError {
                    message: "Failed to get default id".into(),
                }),
            },
            None => Err(JobcanError::ElementExtractError {
                message: "Failed to get default id".into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authenticity_token() {
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
        let token = HtmlExtractor::authenticity_token(&html);

        // Assert
        assert!(token.unwrap() == "token");
    }

    #[test]
    fn working_status_in_working() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <script>
                        var current_status = "working";
                    </script>
                </body>
            </html>"""#;

        // Act
        let status = HtmlExtractor::working_status(body).unwrap();

        // Assert
        assert!(status == WorkingStatus::Working);
    }

    #[test]
    fn working_status_in_not_working() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <script>
                        var current_status = "returned_home";
                    </script>
                </body>
            </html>"""#;

        // Act
        let status = HtmlExtractor::working_status(body).unwrap();

        // Assert
        assert!(status == WorkingStatus::NotWorking);
    }

    #[test]
    fn token() {
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
        let token = HtmlExtractor::token(&html);

        // Assert
        assert!(token.unwrap() == "token");
    }

    #[test]
    fn group_with_multiple_options() {
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
        let expected = vec![
            Group {
                id: "1".to_string(),
                name: "1".to_string(),
            },
            Group {
                id: "2".to_string(),
                name: "2".to_string(),
            },
        ];

        // Act
        let group_ids = HtmlExtractor::groups(&html);

        // Assert
        assert!(group_ids.unwrap() == expected);
    }

    #[test]
    fn group_without_option() {
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
        let group_ids = HtmlExtractor::groups(&html);

        // Assert
        assert!(group_ids.unwrap() == Vec::<Group>::new());
    }

    #[test]
    fn default_group_id_with_expected_text() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <script>
                        var defaultAditGroupId = 1;
                    </script>
                </body>
            </html>"""#;

        // Act
        let status = HtmlExtractor::default_group_id(body).unwrap();

        // Assert
        assert!(status == "1".to_string());
    }

    #[test]
    fn default_group_id_with_unexpected_text() {
        // Arrange
        let body = r#"""
            <html>
                <head></head>
                <body>
                    <script>
                    </script>
                </body>
            </html>"""#;

        // Act
        let status = HtmlExtractor::default_group_id(body);

        // Assert
        assert!(status.is_err());
    }
}
