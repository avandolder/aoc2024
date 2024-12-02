fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reports = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();
    println!("part one: {}", safe_reports);

    let safe_reports_with_dampener = reports
        .iter()
        .filter(|report| {
            (-1..report.len() as i32).any(|index| {
                let mut report = report.to_vec();
                let report = if index >= 0 {
                    report.remove(index as usize);
                    report
                } else {
                    report
                };
                is_report_safe(&report)
            })
        })
        .count();
    println!("part two: {}", safe_reports_with_dampener);

    Ok(())
}

fn is_report_safe(report: &[i32]) -> bool {
    (report.iter().is_sorted() || report.iter().rev().is_sorted())
        && std::iter::zip(report.iter(), &report[1..]).all(|(x, y)| {
            let diff = (*x - *y).abs();
            (1..=3).contains(&diff)
        })
}
