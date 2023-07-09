use std::collections::HashMap;

pub fn creating_accessing() {
  let mut scores: HashMap<String, i32> = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Yellow"), 50);

  let team_name: String = String::from("Blue");
  let score: i32 = scores.get(&team_name).copied().unwrap_or(0);

  println!("{score}");
}

pub fn overwriting() {
  let mut scores: HashMap<String, i32> = HashMap::new();

  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Blue"), 25);

  println!("{:?}", scores);
}

pub fn updating_if_exists() {
  let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

pub fn updating_based_on_value() {
  let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

