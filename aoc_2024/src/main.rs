use aoc_day::AocDayData;

mod aoc_day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod util;

fn main() {
    let aocs: Vec<AocDayData> = vec![
        day1::day1(),
        day2::day2(),
        day3::day3(),
        day4::day4(),
        day5::day(),
    ];

    for aoc in aocs {
        aoc.solve();
    }
}
