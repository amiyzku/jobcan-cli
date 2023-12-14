use anyhow::Result;
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

    pub fn working_status(text: &str) -> WorkingStatus {
        if text.contains("(勤務中)") {
            WorkingStatus::Working
        } else {
            WorkingStatus::NotWorking
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

    pub fn group(html: &Html) -> Result<Vec<Group>> {
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
        let token = JobcanHtmlExtractor::authenticity_token(&html);

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
                    <p>(勤務中)</p>
                </body>
            </html>"""#;

        // Act
        let status = JobcanHtmlExtractor::working_status(body);

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
                    <p></p>
                </body>
            </html>"""#;

        // Act
        let status = JobcanHtmlExtractor::working_status(body);

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
        let token = JobcanHtmlExtractor::token(&html);

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
        let group_ids = JobcanHtmlExtractor::group(&html);

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
        let group_ids = JobcanHtmlExtractor::group(&html);

        // Assert
        assert!(group_ids.unwrap() == Vec::<Group>::new());
    }
}
