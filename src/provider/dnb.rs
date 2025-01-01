use gluer::metadata;
use reqwest::{Client, Url};
use serde::Serialize;
use tracing::info;
use unicode_normalization::UnicodeNormalization;

use crate::error::{Error, Result};

#[metadata]
#[derive(Debug, Default, PartialEq, Serialize)]
pub struct BookData {
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub costs: f64,
}
const URL: &str =
    "https://services.dnb.de/sru/dnb?version=1.1&operation=searchRetrieve&recordSchema=MARC21-xml";

/// Try fetching the book data from the "Deutsche Nationalbibliothek"
pub async fn fetch(client: &Client, isbn: &str) -> Result<BookData> {
    let url = Url::parse_with_params(URL, [("query", &format!("NUM={isbn}"))])
        .map_err(|_| Error::Arguments)?;
    info!("Fetch {url}");
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(Error::Network);
    }
    let text = response.text().await?;
    parse_single(&text, isbn)
}

#[allow(unused)]
pub async fn query(client: &Client, query: &str, page: usize) -> Result<Vec<Record>> {
    let url = Url::parse_with_params(
        URL,
        [
            ("maximumRecords", "100"),
            ("startRecord", format!("{}", page * 100).as_str()),
            ("query", query),
        ],
    )
    .map_err(|_| Error::Arguments)?;

    info!("Fetch {url}");
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(Error::Network);
    }
    let text = response.text().await?;

    // parse the MARC21-xml response
    let document = roxmltree::Document::parse(&text)?;
    let mut results = Vec::new();
    if let Some(records) = document
        .descendants()
        .find(|n| n.tag_name().name() == "records")
    {
        for record in records.children().map(Record::parse) {
            results.push(record);
        }
    }
    Ok(results)
}

const ISBN_COSTS_TAG: &str = "020";
const ISBN_CODE: &str = "a";
const COSTS_CODE: &str = "c";
const EAN_TAG: &str = "024";
const EAN_CODE: &str = "a";
const TITLE_TAG: &str = "245";
const TITLE_CODE: &str = "a";
const SUBTITLE_CODE: &str = "p";
const AUTHOR_TAG: &str = "100";
const AUTHOR_CODE: &str = "a";
const PERSON_TAG: &str = "700";
const PERSON_CODE: &str = "a";
const PUBLISHER_TAG: &str = "264";
const PUBLISHER_CODE: &str = "b";

/// Warning: legacy: 1 DM => 0.51129 EUR
const DM_TO_EUR: f64 = 0.51129;

/// If the title is shorter than this, the subtitle is appended
const SHORT_TITLE_LEN: usize = 16;

/// MARC21 Parsing
///
/// ## See Also
/// https://www.dnb.de/EN/Professionell/Metadatendienste/Datenbezug/SRU/sru_node.html
fn parse_single(response: &str, isbn: &str) -> Result<BookData> {
    let document = roxmltree::Document::parse(response)?;

    let mut first_result = None;

    if let Some(records) = document
        .descendants()
        .find(|n| n.tag_name().name() == "records")
    {
        for record in records.children().map(Record::parse) {
            if record.isbns.iter().any(|e| e == isbn) {
                return Ok(record.data);
            }
            if first_result.is_none() {
                first_result = Some(record.data);
            }
        }
    }

    first_result.ok_or(Error::NothingFound)
}

#[derive(Debug, Default)]
pub struct Record {
    pub isbns: Vec<String>,
    pub data: BookData,
}

impl Record {
    fn parse(record: roxmltree::Node) -> Self {
        let mut data = BookData::default();
        let mut persons = Vec::new();
        let mut isbns = Vec::new();

        let Some(record) = record.children().find(|n| n.has_tag_name("recordData")) else {
            return Self::default();
        };
        let Some(record) = record.children().find(|n| n.has_tag_name("record")) else {
            return Self::default();
        };

        for df in record.children().filter(|x| x.has_tag_name("datafield")) {
            let Some(tag) = df.attribute("tag") else {
                continue;
            };

            match tag {
                ISBN_COSTS_TAG => {
                    if let Some(t) = subfield(df, ISBN_CODE) {
                        isbns.push(t)
                    }
                    if let Some(t) = subfield(df, COSTS_CODE) {
                        data.costs = parse_costs(&t);
                    }
                }
                EAN_TAG => subfield(df, EAN_CODE).map_or((), |t| isbns.push(t)),
                TITLE_TAG => {
                    if let Some(t) = subfield(df, TITLE_CODE) {
                        data.title = t;
                    }
                    // Add subtitle if the title is to short
                    if data.title.len() < SHORT_TITLE_LEN {
                        if let Some(t) = subfield(df, SUBTITLE_CODE) {
                            if !t.is_empty() {
                                data.title.push_str(" - ");
                                data.title.push_str(&t);
                            }
                        }
                    }
                }
                AUTHOR_TAG => {
                    if let Some(t) = subfield(df, AUTHOR_CODE) {
                        data.authors.push(t);
                    }
                }
                PERSON_TAG => {
                    if let Some(t) = subfield(df, PERSON_CODE) {
                        persons.push(t);
                    }
                }
                PUBLISHER_TAG => {
                    if let Some(t) = subfield(df, PUBLISHER_CODE) {
                        data.publisher = t;
                    }
                }
                _ => {}
            };
        }
        if data.authors.is_empty() {
            data.authors = persons;
        }
        // Reformat author names ("<surname>, <forename>" -> "<forename> <surname>")
        for author in &mut data.authors {
            if let Some((s, f)) = author.split_once(',') {
                *author = [f.trim(), s.trim()].join(" ");
            }
        }
        Self { isbns, data }
    }
}

fn subfield(datafield: roxmltree::Node, code: &str) -> Option<String> {
    let subfield = datafield
        .children()
        .find(|n| n.has_tag_name("subfield") && n.attribute("code") == Some(code))?;

    Some(subfield.text()?.nfc().filter(|c| !c.is_control()).collect())
}

fn parse_costs(costs: &str) -> f64 {
    if let Some((_, suffix)) = costs.split_once("EUR ") {
        let num = suffix.split_once(' ').map_or(suffix, |s| s.0);
        num.trim().parse().unwrap_or_default()
    } else if let Some((_, suffix)) = costs.split_once("DM ") {
        let num = suffix.split_once(' ').map_or(suffix, |s| s.0);
        let num: f64 = num.trim().parse().unwrap_or_default();
        (num * DM_TO_EUR * 100.0).round() / 100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_single_record() {
        let response = fs::read_to_string("test/dnb/dnb-response_9783570303337.xml").unwrap();
        let data = parse_single(&response, "9783570303337").unwrap();
        assert_eq!(
            data,
            BookData {
                title: "Eragon - Das Vermächtnis der Drachenreiter".into(),
                authors: vec!["Christopher Paolini".into()],
                publisher: "cbj".into(),
                costs: 9.95,
            }
        )
    }

    #[test]
    fn parse_multiple_records() {
        let response = fs::read_to_string("test/dnb/dnb-response_3440040585.xml").unwrap();
        let data = parse_single(&response, "3440040585").unwrap();
        assert_eq!(
            data,
            BookData {
                title: "Alfred Hitchcock, die drei ??? [Fragezeichen] und der rasende Löwe".into(),
                authors: vec!["Kin Platt".into()],
                publisher: "Franckh".into(),
                costs: 5.01,
            }
        )
    }

    #[test]
    fn parse_no_authors() {
        let response = fs::read_to_string("test/dnb/dnb-response_9783060016150.xml").unwrap();
        let data = parse_single(&response, "9783060016150").unwrap();
        assert_eq!(
            data,
            BookData {
                title: "Das große Tafelwerk interaktiv 2.0".into(),
                authors: vec![
                    "Tilman Pehle".into(), // Herrausgeber
                    "Andreas Gramm".into(),
                    "Hubert König".into(),
                    "Wolfgang Kricke".into(),
                    "Karlheinz Martin".into(),
                    "Lothar Meyer".into(),
                    "Wolfgang Pfeil".into(),
                    "Rolf Winter".into(),
                    "Willi Wörstenfeld".into()
                ],
                publisher: "Cornelsen".into(),
                costs: 12.5,
            }
        )
    }

    #[test]
    fn parse_costs() {
        fn approx_eq(a: f64, b: f64) -> bool {
            (a - b).abs() < 8.0 * f64::EPSILON
        }

        macro_rules! assert_approx_eq {
            ($left:expr, $right:expr $(,)?) => {
                assert!(
                    approx_eq($left, $right),
                    "assertion failed: `left == right`\n  left: `{:?}`,\n right: `{:?}`",
                    $left,
                    $right
                )
            };
        }

        assert_approx_eq!(super::parse_costs("kart. : EUR"), 0.0);
        assert_approx_eq!(super::parse_costs("kart. : EUR 9.95"), 9.95);
        assert_approx_eq!(super::parse_costs("Pp. (nicht im Buchhandel)"), 0.0);
        assert_approx_eq!(
            super::parse_costs("Pp. : EUR 14.95 (DE), EUR 15.40 (AT), sfr 21.90 (freier Pr.)"),
            14.95
        );
        assert_approx_eq!(
            super::parse_costs("Lw. : DM 9.80"),
            (9.80 * DM_TO_EUR * 100.0).round() / 100.0
        );
    }
}
