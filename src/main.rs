use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let reports = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<_>>()
    });

    let (safe_reports, unsafe_reports): (Vec<_>, Vec<_>) =
        reports.partition(|report| is_safe(&report[..]));

    let result = safe_reports.into_iter().count();
    println!("part 1: {}", result);

    let result = result
        + unsafe_reports
            .into_iter()
            .filter(|report| is_safe_tolerate_single_bad_level(report))
            .count();
    println!("part 2: {}", result);
}

fn is_safe(report: &[u8]) -> bool {
    let mut direction = None;

    for &[l1, l2] in report.windows(2).flat_map(<&[u8; 2]>::try_from) {
        if !((1..=3).contains(&(l1.abs_diff(l2)))) {
            return false;
        }

        let current_direction = l2 > l1;

        match direction {
            None => direction = Some(current_direction),
            Some(d) if d != current_direction => {
                return false;
            }
            _ => {}
        }
    }

    return true;
}

fn is_safe_tolerate_single_bad_level(report: &Vec<u8>) -> bool {
    for i in 0..report.len() {
        let mut copy = report.clone();
        copy.remove(i);

        if is_safe(&copy[..]) {
            return true;
        }
    }

    return false;
}
