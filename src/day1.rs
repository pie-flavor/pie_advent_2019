use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn fuel_input(input: &str) -> Vec<u32> {
    input.split('\n').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn calculate_fuel(masses: &[u32]) -> u32 {
    masses.iter().map(|&x| (x / 3).saturating_sub(2)).sum()
}

#[aoc(day1, part2)]
fn calculate_recursive_fuel(masses: &[u32]) -> u32 {
    masses.iter().flat_map(|&x| Fueler(x)).sum()
}

struct Fueler(u32);

impl Iterator for Fueler {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let next = (self.0 / 3).saturating_sub(2);
        if next == 0 {
            None
        } else {
            self.0 = next;
            Some(next)
        }
    }
}
