use strsim::normalized_levenshtein;

use crate::config::{Config, Regret, get_config_path};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Read;

pub fn check_command(command: &str) -> (Option<Regret>, Option<f64>) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(get_config_path().unwrap())
        .unwrap();

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let config: Config = if data.trim().is_empty() {
        Config::default()
    } else {
        toml::from_str(&data).unwrap()
    };

    let norm_input = normalize(command);

    let best_match = config
        .regrets
        .into_iter()
        .map(|r| {
            let score = similarity(&normalize(&r.command), &norm_input);
            (r, score)
        })
        .filter(|(_, score)| *score >= config.warning_threshold)
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

    best_match
        .map(|(r, s)| (Some(r), Some(s)))
        .unwrap_or((None, None))
}

fn similarity(a: &str, b: &str) -> f64 {
    token_jaccard(a, b).max(normalized_levenshtein(a, b))
}

fn token_jaccard(a: &str, b: &str) -> f64 {
    let a_tokens: HashSet<&str> = a.split_whitespace().collect();
    let b_tokens: HashSet<&str> = b.split_whitespace().collect();

    let intersection = a_tokens.intersection(&b_tokens).count();
    let union = a_tokens.union(&b_tokens).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

fn normalize_flag(token: &str) -> &str {
    token.trim_start_matches('-') // strip flags to reduce false negatives
}

fn normalize(command: &str) -> String {
    let mut tokens: Vec<&str> = command.split_whitespace().collect();

    if tokens.len() > 1 {
        tokens[1..].sort_by_key(|t| normalize_flag(t));
    }

    tokens.join(" ")
}
