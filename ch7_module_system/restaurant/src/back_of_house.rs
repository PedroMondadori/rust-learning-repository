pub enum Appetizer {
  Soup,
  Salad,
}

pub struct Breakfast {
  pub toast: String,
  seasonal_fruit: String,
}

impl Breakfast {
  pub fn summer(toast: &str) -> Breakfast {
      return Breakfast {
          toast: String::from(toast),
          seasonal_fruit: get_seasonal_fruit(),
      };
  }
}

fn get_seasonal_fruit() -> String {
  return String::from("peaches");
}

fn fix_incorrect_order() {
  cook_order();
  super::deliver_order();
}

fn cook_order() {}
