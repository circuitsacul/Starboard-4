use rust_fuzzy_search::fuzzy_search_best_n;
use twilight_model::application::command::{CommandOptionChoice, CommandOptionChoiceData};

pub fn best_matches_as_choices(
    query: &str,
    target: &[&str],
    value: Option<fn(&str) -> String>,
) -> Vec<CommandOptionChoice> {
    let best = fuzzy_search_best_n(query, target, 25);

    best.into_iter()
        .map(|item| item.0)
        .map(|name| {
            CommandOptionChoice::String(CommandOptionChoiceData {
                name: name.to_owned(),
                value: value.map_or_else(|| name.to_owned(), |func| func(name)),
                name_localizations: None,
            })
        })
        .collect()
}
