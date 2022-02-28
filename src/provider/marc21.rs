use crate::api;
use crate::provider::BookData;

use unicode_normalization::UnicodeNormalization;

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
pub fn parse(response: &str, isbn: &str) -> api::Result<BookData> {
    let document = roxmltree::Document::parse(response)?;

    let mut first_result = None;

    if let Some(records) = document
        .descendants()
        .find(|n| n.tag_name().name() == "records")
    {
        for record in records.children() {
            let record = parse_record(record);
            if record.isbns.iter().any(|e| e == isbn) {
                return Ok(record.data);
            }
            if first_result.is_none() {
                first_result = Some(record.data);
            }
        }
    }

    first_result.ok_or(api::Error::NothingFound)
}

#[derive(Debug, Default)]
struct Record {
    isbns: Vec<String>,
    data: BookData,
}

fn parse_record(record: roxmltree::Node) -> Record {
    let mut r = Record::default();
    let mut persons = Vec::new();
    for field in record
        .descendants()
        .filter(|x| x.has_tag_name("datafield") && x.has_attribute("tag"))
    {
        match field.attribute("tag").unwrap() {
            ISBN_COSTS_TAG => {
                subfield(field, ISBN_CODE).map_or((), |t| r.isbns.push(t));
                subfield(field, COSTS_CODE).map_or((), |t| r.data.costs = parse_costs(&t))
            }
            EAN_TAG => subfield(field, EAN_CODE).map_or((), |t| r.isbns.push(t)),
            TITLE_TAG => {
                subfield(field, TITLE_CODE).map_or((), |t| r.data.title = t);
                // Add subtitle if the title is to short
                if r.data.title.len() < SHORT_TITLE_LEN {
                    subfield(field, SUBTITLE_CODE).map_or((), |t| {
                        if !t.is_empty() {
                            r.data.title.push_str(" - ");
                            r.data.title.push_str(&t);
                        }
                    });
                }
            }
            AUTHOR_TAG => subfield(field, AUTHOR_CODE).map_or((), |t| r.data.authors.push(t)),
            PERSON_TAG => subfield(field, PERSON_CODE).map_or((), |t| persons.push(t)),
            PUBLISHER_TAG => subfield(field, PUBLISHER_CODE).map_or((), |t| r.data.publisher = t),
            _ => {}
        };
    }
    if r.data.authors.is_empty() {
        r.data.authors = persons;
    }
    // Reformat author names ('<forename> <surname>')
    for author in &mut r.data.authors {
        *author = author
            .rsplit(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join(" ");
    }
    r
}

fn subfield(datafield: roxmltree::Node, code: &str) -> Option<String> {
    let subfield = datafield
        .children()
        .find(|n| n.has_tag_name("subfield") && n.attribute("code") == Some(code))?;

    Some(subfield.text()?.nfc().filter(|c| !c.is_control()).collect())
}

fn parse_costs(costs: &str) -> f64 {
    if let Some((_, suffix)) = costs.split_once("EUR ") {
        let num = suffix.split_once(' ').map(|s| s.0).unwrap_or(suffix);
        num.trim().parse().unwrap_or_default()
    } else if let Some((_, suffix)) = costs.split_once("DM ") {
        let num = suffix.split_once(' ').map(|s| s.0).unwrap_or(suffix);
        let num: f64 = num.trim().parse().unwrap_or_default();
        (num * DM_TO_EUR * 100.0).round() / 100.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parse_single_record() {
        let response = fs::read_to_string("test/data/dnb/dnb-response_9783570303337.xml").unwrap();
        let data = parse(&response, "9783570303337").unwrap();
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
        let response = fs::read_to_string("test/data/dnb/dnb-response_3440040585.xml").unwrap();
        let data = parse(&response, "3440040585").unwrap();
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
        let response = fs::read_to_string("test/data/dnb/dnb-response_9783060016150.xml").unwrap();
        let data = parse(&response, "9783060016150").unwrap();
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
