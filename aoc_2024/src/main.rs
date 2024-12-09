use aoc_day::AocDayData;

mod aoc_day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod util;

fn main() {
    let aocs: Vec<AocDayData> = vec![
        day1::day1(),
        day2::day2(),
        day3::day3(),
        day4::day4(),
        day5::day(),
        day6::day(),
        day7::day(),
    ];

    for aoc in aocs {
        aoc.solve();
    }
}
