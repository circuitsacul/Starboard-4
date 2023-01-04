use std::{
    collections::HashMap,
    fmt::Write,
    time::{Duration, Instant},
};

use sqlx::{postgres::PgRow, Column, Executor, Row, ValueRef};
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{
    client::bot::StarboardBot, concat_format, errors::StarboardResult,
    owner::code_block::parse_code_blocks,
};

pub async fn run_sql(bot: &StarboardBot, event: &MessageCreate) -> StarboardResult<()> {
    bot.http.create_typing_trigger(event.channel_id).await?;

    let blocks = parse_code_blocks(event.content.strip_prefix("star sql").unwrap());
    let mut results = Vec::new();

    let mut tx = bot.pool.begin().await?;
    for (code, meta) in &blocks {
        let return_results = meta.get("return").is_some();
        let total_execs = meta.get("runs").map_or(1, |v| v.parse().unwrap()).max(1);

        let mut result = None;
        let mut execution_times = Vec::new();
        for _ in 0..total_execs {
            let elapsed = if return_results {
                let start = Instant::now();
                let rows = tx.fetch_all(code.as_str()).await?;
                let elapsed = start.elapsed();
                result.replace(Some(rows.into_iter().map(row_to_json).collect()));
                elapsed
            } else {
                let start = Instant::now();
                tx.execute(code.as_str()).await?;
                start.elapsed()
            };
            execution_times.push(elapsed);
        }

        let result = SqlResult {
            execution_times,
            inspect: result.unwrap_or(None),
            tag: meta.get("tag").unwrap_or(&"?query?").to_string(),
        };
        results.push(result);
    }
    tx.rollback().await?;

    let mut final_result = String::new();
    for result in results {
        final_result.push_str(&concat_format!(
            "Query {} ran {} times, " <- result.tag, result.execution_times.len();
            "with an average time of {:?}.\n" <- result.average_time();
        ));

        if let Some(inspect) = result.inspect {
            final_result.push_str("```rs\n");
            let mut x = 0;
            for row in inspect {
                x += 1;
                if x == 5 {
                    final_result.push_str(" - and more...");
                    break;
                }
                writeln!(final_result, " - {row:?}").unwrap();
            }
            final_result.push_str("```\n");
        }
    }

    bot.http
        .create_message(event.channel_id)
        .content(&final_result)
        .unwrap()
        .await?;
    Ok(())
}

fn row_to_json(row: PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = if value.is_null() {
            "NULL".to_string()
        } else {
            value.as_str().unwrap().to_string()
        };
        result.insert(col.name().to_string(), value);
    }

    result
}

#[derive(Debug)]
struct SqlResult {
    pub execution_times: Vec<Duration>,
    pub inspect: Option<Vec<HashMap<String, String>>>,
    pub tag: String,
}

impl SqlResult {
    pub fn average_time(&self) -> Duration {
        let mut total = Duration::new(0, 0);
        for time in &self.execution_times {
            total += *time;
        }
        total.div_f64(self.execution_times.len() as f64)
    }
}
