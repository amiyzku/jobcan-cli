use anyhow::Result;
use regex::Regex;
use scraper::Html;

use crate::working_status::WorkingStatus;

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
        let re = Regex::new(r#"var current_status = "(.*?)";"#).unwrap();
        match re.captures(text) {
            Some(caps) => match caps.get(1).unwrap().as_str() {
                "returned_home" => Ok(WorkingStatus::NotWorking),
                "working" => Ok(WorkingStatus::Working),
                "resting" => Ok(WorkingStatus::Resting),
                _ => anyhow::bail!("Unknown working status"),
            },
            None => anyhow::bail!("Failed to get working status"),
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

    pub fn groups(html: &Html) -> Result<Vec<Group>> {
        let selector = scraper::Selector::parse("#adit_group_id > option").unwrap();
        let options = html.select(&selector);
        let group_ids = options
            .map(|option| {
                let id = option
                    .value()
                    .attr("value")
                    .expect("Failed to get value of group id")
                    .to_string();
                let name = option
                    .text()
                    .next()
                    .expect("Failed to get value of group name")
                    .to_string();
                Group { id, name }
            })
            .collect();
        Ok(group_ids)
    }

    pub fn default_group_id(text: &str) -> Result<String> {
        let re = Regex::new(r#"var defaultAditGroupId = (.*?);"#).unwrap();
        match re.captures(text) {
            Some(caps) => Ok(caps.get(1).unwrap().as_str().to_string()),
            None => anyhow::bail!("Failed to get default id"),
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
