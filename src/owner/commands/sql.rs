use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use sqlx::{postgres::PgRow, Column, Row};
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{client::bot::StarboardBot, owner::code_block::parse_code_blocks};

pub async fn run_sql(bot: &StarboardBot, event: &MessageCreate) -> anyhow::Result<()> {
    let blocks = parse_code_blocks(&event.content.strip_prefix("star sql").unwrap());
    let mut results = Vec::new();

    for (code, meta) in blocks.iter() {
        let return_results = meta.get("return").is_some();
        let total_execs = meta.get("runs").map(|v| v.parse().unwrap()).unwrap_or(1);

        let mut result = None;
        let mut execution_times = Vec::new();
        for _ in 0..total_execs {
            let start = Instant::now();
            match return_results {
                true => {
                    let query = sqlx::query(&code);
                    // let query_str = format!(
                    //    "with result as ({code}) select json_agg(result) from result;"
                    //);
                    //let query = sqlx::query(&query_str);
                    let rows = query.fetch_all(&*bot.pool).await?;
                    result.replace(Some(rows.into_iter().map(|r| row_to_json(r)).collect()));
                }
                false => {
                    let query = sqlx::query(&code);
                    query.execute(&*bot.pool).await?;
                    result.replace(None);
                }
            }
            execution_times.push(start.elapsed());
        }

        let result = SqlResult {
            execution_times,
            inspect: result.unwrap_or(None),
        };
        results.push(result);
    }

    println!("{:#?}", results);
    Ok(())
}

fn row_to_json(row: PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let name = col.name();
        result.insert(name.to_string(), name.to_string());
    }

    result
}

#[derive(Debug)]
struct SqlResult {
    pub execution_times: Vec<Duration>,
    pub inspect: Option<Vec<HashMap<String, String>>>,
}

impl SqlResult {
    pub fn average_time(&self) -> f64 {
        self.execution_times
            .iter()
            .map(|d| d.as_secs_f64())
            .sum::<f64>()
            / self.execution_times.len() as f64
    }
}
