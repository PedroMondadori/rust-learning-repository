pub fn initializtion() {
    let _vec1: Vec<i32> = Vec::new();
    let _vec2: Vec<i32> = vec![1, 2, 3];
    
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

pub fn read_elements() {
  let v: Vec<i32> = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("The third element is {third}");

  let third: Option<&i32> = v.get(2);
  match third {
    Some(num) => println!("The thid element is {num}"),
    None => println!("There is no third element."),      
  }
}

pub fn mutability() {
  let v: Vec<i32> = vec![1,2,3,4,5];
  // let mut v: Vec<i32> = vec![1,2,3,4,5];

  let first: &i32 = &v[0];

  // uncommenting the following line causes the program not to compile
  // so that the previous reference to v remains valid after altering it
  // v.push(6);

  println!("The first element is: {first}");
}

pub fn iterating() {
  let v: Vec<i32> = vec![100, 32, 57];
    for i in &v {
        println!("{}", *i);
    }

    let mut v: Vec<i32> = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

pub fn enums() {
  #[derive(Debug)]
  enum SpredsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row: Vec<SpredsheetCell> = vec![
    SpredsheetCell::Int(3),
    SpredsheetCell::Text(String::from("blue")),
    SpredsheetCell::Float(10.12),
  ];

  dbg!(row);
}