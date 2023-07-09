pub fn creating() {
  let _s1: String = String::new();

  // let data: &str = "initial contents";
  // let s: String = data.to_string();
  let s2: String = "second string".to_string();

  let s3: String = String::from("third string");

  for string in [&s2, &s3] {
    println!("{string}");
  }
}

pub fn updating() {
  let mut s: String = String::from("foo");
  s.push_str("bar");

  println!("{s}");
}

pub fn concatenation() {
  let s1: String = String::from("Hello, ");
  let s2: String = String::from("world!");
  // the '+' operator receives 'self' as its first argument, thus taking ownership of s1
  let _s3: String = s1 + &s2; // note s1 has been moved here and can no longer be used

  let s1: String = String::from("tic");
  let s2: String = String::from("tac");
  let s3: String = String::from("toe");

  // bad way to concatenate various strings
  // let _s: String = s1 + "-" + &s2 + "-" + &s3;

  // better
  let s: String = format!("{s1}-{s2}-{s3}");

  println!("{s}");
}

pub fn iterating() {
  let hello = "Здравствуйте";

  for c in hello.chars() {
    println!("{c}");
  }

  for b in hello.bytes() {
    println!("{b}");
  }
}