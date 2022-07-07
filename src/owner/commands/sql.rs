use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use sqlx::{postgres::PgRow, Column, Executor, Row, ValueRef};
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{client::bot::StarboardBot, concat_format, owner::code_block::parse_code_blocks};

pub async fn run_sql(bot: &StarboardBot, event: &MessageCreate) -> anyhow::Result<()> {
    bot.http
        .create_typing_trigger(event.channel_id)
        .exec()
        .await?;

    let blocks = parse_code_blocks(&event.content.strip_prefix("star sql").unwrap());
    let mut results = Vec::new();

    for (code, meta) in blocks.iter() {
        let return_results = meta.get("return").is_some();
        let total_execs = meta
            .get("runs")
            .map(|v| v.parse().unwrap())
            .unwrap_or(1)
            .max(1);

        let mut result = None;
        let mut execution_times = Vec::new();
        for _ in 0..total_execs {
            let elapsed = match return_results {
                true => {
                    let start = Instant::now();
                    let rows = bot.pool.fetch_all(code.as_str()).await?;
                    let elapsed = start.elapsed();
                    result.replace(Some(rows.into_iter().map(|r| row_to_json(r)).collect()));
                    elapsed
                }
                false => {
                    let query = sqlx::query(&code);
                    let start = Instant::now();
                    query.execute(&*bot.pool).await?;
                    start.elapsed()
                }
            };
            execution_times.push(elapsed);
        }

        let result = SqlResult {
            execution_times,
            inspect: result.unwrap_or(None),
            tag: meta.get("tag").unwrap_or_else(|| &"?query?").to_string(),
        };
        results.push(result);
    }

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
                final_result.push_str(&format!(" - {:?}\n", row));
            }
            final_result.push_str("```\n")
        }
    }

    bot.http
        .create_message(event.channel_id)
        .content(&final_result)
        .unwrap()
        .exec()
        .await?;
    Ok(())
}

fn row_to_json(row: PgRow) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for col in row.columns() {
        let value = row.try_get_raw(col.ordinal()).unwrap();
        let value = match value.is_null() {
            true => "NULL".to_string(),
            false => value.as_str().unwrap().to_string(),
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
        for time in self.execution_times.iter() {
            total += *time;
        }
        total.div_f64(self.execution_times.len() as f64)
    }
}
