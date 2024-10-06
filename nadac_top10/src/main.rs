use std::{
    fmt::{self, Display, Formatter},
    sync::Arc,
};

use anyhow::Error;
use csv::{ReaderBuilder, Trim};
use rust_decimal::prelude::*;
use serde::Deserialize;
use tokio::sync::RwLock;

const DOCUMENT_URL: &str = "https://download.medicaid.gov/data/nadac-comparison-04-17-2024.csv";
const DEFAULT_YEAR: &str = "2023";
const NUM_OF_RECORDS: usize = 10;
const PRECISION: u32 = 2;

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct Record {
    #[serde(rename(deserialize = "NDC Description"))]
    description: String,
    #[serde(rename(deserialize = "NDC"))]
    ndc: String,
    #[serde(rename(deserialize = "Old NADAC Per Unit"))]
    price_old: Decimal,
    #[serde(rename(deserialize = "New NADAC Per Unit"))]
    price_new: Decimal,
    #[serde(rename(deserialize = "Classification for Rate Setting"))]
    classification: String,
    #[serde(rename(deserialize = "Percent Change"))]
    percent_change: String,
    #[serde(rename(deserialize = "Primary Reason"))]
    reason: String,
    #[serde(rename(deserialize = "Start Date"))]
    start_date: String,
    #[serde(rename(deserialize = "End Date"))]
    end_date: String,
    #[serde(rename(deserialize = "Effective Date"))]
    effective_date: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct ReportRow {
    description: String,
    price_change: Decimal,
}

impl From<&Record> for ReportRow {
    fn from(rec: &Record) -> Self {
        ReportRow {
            description: rec.description.clone(),
            price_change: rec.price_new - rec.price_old,
        }
    }
}

impl Display for ReportRow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut price = self.price_change;
        price.rescale(PRECISION);
        let price_str = if price.is_sign_negative() {
            format!("-${}", price.abs())
        } else {
            format!("${}", price)
        };
        writeln!(f, "{}: {}", price_str, self.description)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TypeOfEntries {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LimitedVector {
    type_of_entries: TypeOfEntries,
    entries: Vec<ReportRow>,
}

impl LimitedVector {
    fn needs_update(&self, row: ReportRow) -> bool {
        if self.entries.len() < NUM_OF_RECORDS {
            true
        } else {
            match self.type_of_entries {
                TypeOfEntries::High => {
                    self.entries.first().unwrap().price_change < row.price_change
                        && !self.entries.contains(&row)
                }
                TypeOfEntries::Low => {
                    self.entries.last().unwrap().price_change > row.price_change
                        && !self.entries.contains(&row)
                }
            }
        }
    }

    fn add_record(&mut self, row: ReportRow) {
        if self.entries.len() < NUM_OF_RECORDS {
            self.entries.push(row);
        } else {
            match self.type_of_entries {
                TypeOfEntries::High => {
                    if self.entries.first().unwrap().price_change < row.price_change
                        && !self.entries.contains(&row)
                    {
                        self.entries.remove(0);
                        self.entries.push(row);
                    } else {
                        return;
                    }
                }
                TypeOfEntries::Low => {
                    if self.entries.last().unwrap().price_change > row.price_change
                        && !self.entries.contains(&row)
                    {
                        self.entries.remove(NUM_OF_RECORDS - 1);
                        self.entries.push(row);
                    } else {
                        return;
                    }
                }
            }
        }
        self.entries
            .sort_by(|a, b| a.price_change.cmp(&b.price_change));
    }
}

impl Display for LimitedVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.type_of_entries {
            TypeOfEntries::High => {
                let records: Vec<String> = self
                    .entries
                    .iter()
                    .rev()
                    .map(|entry| format!("{}", entry))
                    .collect();
                write!(f, "{}", records.join(""))
            }
            TypeOfEntries::Low => {
                let records: Vec<String> = self
                    .entries
                    .iter()
                    .map(|entry| format!("{}", entry))
                    .collect();
                write!(f, "{}", records.join(""))
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Report {
    year: String,
    top: Arc<RwLock<LimitedVector>>,
    bottom: Arc<RwLock<LimitedVector>>,
}

impl Report {
    fn new(report_year: String) -> Self {
        Report {
            year: report_year,
            top: Arc::new(RwLock::new(LimitedVector {
                type_of_entries: TypeOfEntries::High,
                entries: Vec::new(),
            })),
            bottom: Arc::new(RwLock::new(LimitedVector {
                type_of_entries: TypeOfEntries::Low,
                entries: Vec::new(),
            })),
        }
    }

    async fn report_to_string(&self) -> String {
        println!();
        let top_title = format!(
            "Top {} NADAC per unit price increases of {}:\n",
            NUM_OF_RECORDS, self.year
        );
        let bottom_title = format!(
            "Top {} NADAC per unit price decreases of {}:\n",
            NUM_OF_RECORDS, self.year
        );

        format!(
            "{top_title}{}\n{bottom_title}{}",
            self.top.read().await,
            self.bottom.read().await
        )
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Error> {
    if let Ok(report) = run_report(DEFAULT_YEAR).await {
        let result = report.report_to_string().await;
        println!("{}", result);
    }
    Ok(())
}

async fn run_report(year: &str) -> Result<Report, Error> {
    let report = Report::new(year.to_string());
    _ = tokio::join!(process_data(&report));
    Ok(report)
}

async fn process_data(report: &Report) -> Result<(), Error> {
    let mut response = reqwest::get(DOCUMENT_URL).await?;
    let mut is_first_iteration = true;
    let mut section = String::new();
    while let Some(chunk) = response.chunk().await? {
        let s = String::from_utf8_lossy(&chunk);
        let parse_to_here = s.rfind('\n').unwrap();
        let parse_this = s.get(0..parse_to_here).unwrap();
        section.push_str(parse_this);
        let report_c = report.clone();
        tokio::spawn(async move {
            process_records(section.as_bytes(), is_first_iteration, &report_c).await;
        });
        section = s.get(parse_to_here + 1..).unwrap().to_string();
        is_first_iteration = false;
    }
    Ok(())
}

async fn process_records(section_bytes: &[u8], has_headers: bool, report: &Report) {
    let mut rdr = ReaderBuilder::new()
        .has_headers(has_headers)
        .trim(Trim::All)
        .from_reader(section_bytes);
    for result in rdr.deserialize::<Record>() {
        match result {
            Ok(record) => {
                process_record(record, report).await;
            }
            Err(e) => {
                println!("ERROR: {:?}", e);
            }
        }
    }
}

#[inline]
async fn process_record(record: Record, report: &Report) {
    if record.effective_date.ends_with(report.year.as_str()) {
        let report_row = ReportRow::from(&record);

        if report.top.read().await.needs_update(report_row.clone()) {
            report.top.write().await.add_record(report_row.clone());
        }
        if report.bottom.read().await.needs_update(report_row.clone()) {
            report.bottom.write().await.add_record(report_row.clone());
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[tokio::test]
    async fn test_ok() {
        let report_year = "2020";
        let expected_output: String = fs::read_to_string("./data/top_10_2020.txt").unwrap();

        let report = run_report(report_year).await.unwrap();
        let output = report.report_to_string().await;

        assert_eq!(expected_output, output);
    }

    #[tokio::test]
    async fn test_no_data() {
        let report_year = "1955"; // some year likely not in the data file
        let report = run_report(report_year).await.unwrap();

        assert_eq!(report_year, report.year);
        assert!(report.top.read().await.entries.is_empty());
        assert!(report.bottom.read().await.entries.is_empty());
    }
}
