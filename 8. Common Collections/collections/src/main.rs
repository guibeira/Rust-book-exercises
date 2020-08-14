fn main() {
    use std::collections::HashMap;
    
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let score_blue_team = scores.get(&"Blue".to_string());
    for (key, value) in &scores {
        println!("{}: {}", key, value);

    }

}

