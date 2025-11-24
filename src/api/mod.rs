use crate::PuzzleId;
use reqwest::{
    blocking::{Client, Response},
    header,
};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

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
