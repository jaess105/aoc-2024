pub trait AocDay {
    fn get_day_number(&self) -> u8;
    fn get_file_path(&self) -> String;
    fn solve_a(&self, input: String) -> i32;
    fn solve_b(&self, input: String) -> i32;
}
