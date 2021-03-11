fn main() {
    let numbers = "1 2 3 4 5";

    high_and_low(numbers);
}

fn high_and_low(numbers: &str) {
    let numbers_split = numbers.split(" ");
    let parsed = numbers_split.map(|x| str::parse::<i32>(x));

    let mut high: i32 = 0;
    let mut low: i32 = 0;
  }