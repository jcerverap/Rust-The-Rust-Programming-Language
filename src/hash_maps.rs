pub fn do_hash_maps() {

    println!("--- Hash Maps Exercise ---");

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (team, score) in &scores {
        println!("Team: {}, Score: {}", team, score);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(&s) => println!("Score for team {}: {}", team_name, s),
        None => println!("No score found for team {}", team_name),
    }

    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(25);

    println!("Updated Scores:");
    for (team, score) in &scores {
        println!("Team: {}, Score: {}", team, score);
    }

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    println!("{scores2:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");    
}