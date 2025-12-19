use crate::PuzzleId;
use reqwest::{
    blocking::{Client, Response},
    header::{self, HeaderMap, HeaderValue, InvalidHeaderValue},
};
use scraper::{Html, Selector};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

const EMAIL: &str = "emil@englesson.net";

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

        let client = client.default_headers(default_headers(session)?);

        let client = client.build()?;
        Ok(Self { client })
    }

    fn get_text(&self, url: &str) -> Result<String, ApiError> {
        let text = self.client.get(url).send().and_then(Response::text)?;
        Ok(text)
    }

    pub fn get_input(&self, id: &PuzzleId) -> Result<String, ApiError> {
        if !is_day_unlocked(id) {
            return Err(ApiError::DayLocked);
        }
        self.get_text(&input_url(id))
    }

    pub fn get_puzzle(&self, id: &PuzzleId) -> Result<PuzzlePrompt, ApiError> {
        if !is_day_unlocked(id) {
            return Err(ApiError::DayLocked);
        }
        let html = self.get_text(&to_url(id))?;
        let puzzles = self.extract_puzzles(&html)?;
        puzzles
            .into_iter()
            .nth(id.part.saturating_sub(1) as usize)
            .ok_or(ApiError::ParseError)
    }

    fn extract_puzzles(&self, html: &str) -> Result<Vec<PuzzlePrompt>, ApiError> {
        let document = Html::parse_document(html);

        let article_selector = Selector::parse("article.day-desc").unwrap();
        let articles: Vec<_> = document.select(&article_selector).collect();

        if articles.is_empty() {
            return Err(ApiError::ParseError);
        }

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

        Ok(articles
            .into_iter()
            .map(|article| PuzzlePrompt {
                description: article.html(),
                styles: styles.clone(),
            })
            .collect())
    }

    // Return an enum instead of a string (Correct, TooHigh, TooLow, RateLimit)?
    pub fn submit(&self, id: &PuzzleId, solution: String) -> Result<String, ApiError> {
        if !is_day_unlocked(id) {
            return Err(ApiError::DayLocked);
        }
        let params = [("level", id.part.to_string()), ("answer", solution)];
        let response = self.client.post(answer_url(id)).form(&params).send()?;
        let text = response.text()?;
        Ok(text)
    }
}

fn is_day_unlocked(id: &PuzzleId) -> bool {
    let est_offset = UtcOffset::from_hms(-5, 0, 0).unwrap();
    let est_now = OffsetDateTime::now_utc().to_offset(est_offset);

    let unlock_date = Date::from_calendar_date(
        id.year.cast_signed(),
        Month::December,
        u8::try_from(id.day).unwrap(),
    )
    .unwrap()
    .with_time(Time::MIDNIGHT)
    .assume_offset(est_offset);

    est_now >= unlock_date
}

fn default_headers(session: &str) -> Result<HeaderMap, InvalidHeaderValue> {
    let cookie = format!("session={session}");
    let mut cookie_header = HeaderValue::try_from(&cookie)?;
    cookie_header.set_sensitive(true);

    let user_agent = HeaderValue::try_from(&user_agent())?;

    let email = HeaderValue::try_from(EMAIL)?;

    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, cookie_header);
    headers.insert(header::USER_AGENT, user_agent);
    headers.insert(header::FROM, email);

    Ok(headers)
}

fn user_agent() -> String {
    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");

    assert!(!repo.is_empty());
    assert!(!version.is_empty());

    format!("{repo}@{version} (contact: {EMAIL})")
}

#[must_use]
fn to_url(id: &PuzzleId) -> String {
    format!("https://adventofcode.com/{}/day/{}", id.year, id.day)
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
    DayLocked,
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
