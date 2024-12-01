use day1::Day1;

pub mod day1;

fn main() {
    let result = Day1::get_answer_part_2();
    println!("{result}")
}

pub trait Day<T> {
    fn get_answer_part_1() -> T;
    fn get_answer_part_2() -> T;
}