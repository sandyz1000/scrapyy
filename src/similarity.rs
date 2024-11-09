use std::collections::HashMap;

use crate::error::{AppResult, Error};

fn are_args_valid(main_string: &str, target_strings: &[String]) -> bool {
    !main_string.is_empty()
        && !target_strings.is_empty()
        && target_strings.iter().all(|s| !s.is_empty())
}

fn compare_two_strings(first: &str, second: &str) -> f64 {
    let first = first.replace(char::is_whitespace, "");
    let second = second.replace(char::is_whitespace, "");

    if first == second {
        return 1.0; // identical or empty
    }
    if first.len() < 2 || second.len() < 2 {
        return 0.0; // if either is a 0-letter or 1-letter string
    }

    let mut first_bigrams = HashMap::new();
    for i in 0..first.len() - 1 {
        let bigram = &first[i..i + 2];
        *first_bigrams.entry(bigram).or_insert(0) += 1;
    }

    let mut intersection_size = 0;
    for i in 0..second.len() - 1 {
        let bigram = &second[i..i + 2];
        if let Some(count) = first_bigrams.get_mut(bigram) {
            if *count > 0 {
                *count -= 1;
                intersection_size += 1;
            }
        }
    }

    (2.0 * intersection_size as f64) / (first.len() + second.len() - 2) as f64
}

#[derive(Debug, Clone)]
pub struct MatchResult {
    pub target: String,
    pub rating: f64,
}

pub struct BestMatch {
    pub ratings: Vec<MatchResult>,
    pub best_match: MatchResult,
    pub best_match_index: usize,
}

pub fn find_best_match(main_string: &str, targets: &Vec<String>) -> AppResult<BestMatch> {
    if !are_args_valid(main_string, targets) {
        let msg = "Bad arguments: First argument should be a string, second should be an array of strings".to_string();
        return Err(Error::AppError(msg));
    }

    let mut ratings = Vec::new();
    let mut best_match_index = 0;

    for (i, target) in targets.iter().enumerate() {
        let current_rating = compare_two_strings(main_string, target);
        ratings.push(MatchResult {
            target: target.clone(),
            rating: current_rating,
        });
        if current_rating > ratings[best_match_index].rating {
            best_match_index = i;
        }
    }

    let best_match = ratings[best_match_index].clone();

    Ok(BestMatch {
        ratings,
        best_match,
        best_match_index,
    })
}

#[test]
fn test_similarity() {
    let main_string = "hello world";
    let target_strings = vec!["hello", "world", "hello world", "hell"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    match find_best_match(main_string, &target_strings) {
        Ok(BestMatch {
            ratings,
            best_match,
            best_match_index,
        }) => {
            println!("Ratings: {:?}", ratings);
            println!("Best Match: {:?}", best_match);
            println!("Best Match Index: {:?}", best_match_index);
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
