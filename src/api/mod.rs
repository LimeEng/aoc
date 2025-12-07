use crate::PuzzleId;
use reqwest::{
    blocking::{Client, Response},
    header,
};
use scraper::{Html, Selector};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

pub struct PuzzlePrompt {
    pub description: String,
    pub styles: String,
}

pub struct AdventOfCode {
    client: Client,
}

impl AdventOfCode {
    pub fn new(session: &str) -> Result<Self, ApiError> {
        let client = reqwest::blocking::ClientBuilder::new();

        let mut session = header::HeaderValue::from_str(session)?;
        session.set_sensitive(true);
        let user_agent = header::HeaderValue::from_str(&user_agent())?;

        let mut headers = header::HeaderMap::new();
        headers.insert(header::COOKIE, session);
        headers.insert(header::USER_AGENT, user_agent);

        let client = client.default_headers(headers);

        let client = client.build()?;
        Ok(Self { client })
    }

    fn get_text(&self, url: &str) -> Result<String, ApiError> {
        let text = self.client.get(url).send().and_then(Response::text)?;
        Ok(text)
    }

    pub fn get_puzzle(&self, _id: &PuzzleId) -> Result<String, ApiError> {
        todo!()
    }

    pub fn get_input(&self, id: &PuzzleId) -> Result<String, ApiError> {
        self.get_text(&input_url(id))
    }

    pub fn part_1_prompt(&self, year: u32, day: u32) -> Result<PuzzlePrompt, ApiError> {
        let html = self.get_text(&day_url(year, day))?;
        self.extract_part_prompt(&html, 1)
    }

    pub fn part_2_prompt(&self, year: u32, day: u32) -> Result<PuzzlePrompt, ApiError> {
        let html = self.get_text(&day_url(year, day))?;
        self.extract_part_prompt(&html, 2)
    }

    fn extract_part_prompt(&self, html: &str, part: u32) -> Result<PuzzlePrompt, ApiError> {
        let document = Html::parse_document(html);

        let article_selector = Selector::parse("article.day-desc").unwrap();
        let article = document
            .select(&article_selector)
            .nth((part - 1) as usize)
            .ok_or(ApiError::ParseError)?;

        let description = article.html();

        // Fetch external stylesheets (only default, not alternate)
        let link_selector = Selector::parse("link[rel=\"stylesheet\"]").unwrap();
        let styles = document
            .select(&link_selector)
            .filter(|link| {
                // Exclude alternate stylesheets
                link.value().attr("rel") == Some("stylesheet")
            })
            .filter_map(|link| link.value().attr("href"))
            .filter_map(|href| {
                let css_url = if href.starts_with("http") {
                    href.to_string()
                } else {
                    format!("https://adventofcode.com{href}")
                };
                self.get_text(&css_url).ok()
            })
            .collect::<Vec<_>>()
            .join("\n\n");

        Ok(PuzzlePrompt {
            description,
            styles,
        })
    }

    pub fn submit(&self, id: &PuzzleId, solution: String) -> Result<String, ApiError> {
        let params = [("level", id.part.to_string()), ("answer", solution)];
        let response = self.client.post(answer_url(id)).form(&params).send()?;

        todo!()
    }

    fn is_day_unlocked(id: &PuzzleId) -> bool {
        let est_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();
        let est_now = OffsetDateTime::now_utc().to_offset(est_offset);

        let unlock_date = Date::from_calendar_date(id.year as i32, Month::December, id.day as u8)
            .unwrap()
            .with_time(Time::MIDNIGHT)
            .assume_offset(est_offset);

        est_now >= unlock_date
    }
}

fn user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    format!("{repo}@{version} by {authors}")
}

#[must_use]
fn to_url(id: &PuzzleId) -> String {
    format!("https://adventofcode.com/{}/day/{}", id.year, id.day)
}

#[must_use]
fn day_url(year: u32, day: u32) -> String {
    format!("https://adventofcode.com/{year}/day/{day}")
}

#[must_use]
fn input_url(id: &PuzzleId) -> String {
    format!("{}/input", to_url(id))
}

#[must_use]
fn answer_url(id: &PuzzleId) -> String {
    format!("{}/answer", to_url(id))
}

#[derive(Debug)]
pub enum ApiError {
    InvalidHeader(header::InvalidHeaderValue),
    Reqwest(reqwest::Error),
    ParseError,
}

impl From<header::InvalidHeaderValue> for ApiError {
    fn from(error: header::InvalidHeaderValue) -> Self {
        ApiError::InvalidHeader(error)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::Reqwest(error)
    }
}
