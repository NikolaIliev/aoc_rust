use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

fn new_day(year: &str, day: &str) {
    let path_base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path_rs = path_base.join(format!("src/bin/year{}_day{}.rs", year, day));
    let path_input = path_base.join(format!("inputs/year{}_day{}.txt", year, day));

    if path_rs.exists() || path_input.exists() {
        println!("Task already exists, exiting without changing anything.");

        return;
    }

    fs::write(
        &path_rs,
        r#"
use std::time::Instant;

use aoc_rust::read_input;

fn part_1(input: &str) -> String {
    return "".to_string();
}

fn part_2(input: &str) -> String {
    return "".to_string();
}

fn main() {
    let input = read_input("YEAR", "DAY");

    let start_part_1 = Instant::now();
    let part_1_result = part_1(&input);
    let part_1_time = start_part_1.elapsed();

    println!();
    println!("Part 1: {} ({:?})", part_1_result, part_1_time);

    let start_part_2 = Instant::now();
    let part_2_result = part_2(&input);
    let part_2_time = start_part_2.elapsed();

    println!("Part 2: {} ({:?})", part_2_result, part_2_time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_part_1() {
        let input = r"".trim();

        assert_eq!(part_1(input), "");
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let input = r"".trim();

        assert_eq!(part_2(input), "");
    }
}
"#
        .replace("YEAR", year)
        .replace("DAY", day)
        .trim(),
    )
    .unwrap();

    println!("Created {}", path_rs.display());

    fs::write(&path_input, "").unwrap();
    println!("Created {}", path_input.display());
}

fn check_cargo_watch_installed() -> bool {
    return Command::new("which")
        .args(["cargo-watch"])
        .status()
        .unwrap()
        .success();
}

fn install_cargo_watch() {
    println!("cargo-watch missing. Installing now...");
    sleep(Duration::from_secs(3));

    Command::new("cargo")
        .args(["install", "cargo-watch"])
        .stdout(Stdio::inherit())
        .status()
        .unwrap();

    println!("cargo-watch installed. Please run command again");
}

fn test(year: &str, day: &str) {
    if !check_cargo_watch_installed() {
        install_cargo_watch()
    } else {
        Command::new("cargo-watch")
            .args(["-x", &format!("test --bin year{}_day{}", year, day)])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap();
    }
}

fn run(year: &str, day: &str) {
    Command::new("cargo")
        .args([
            "run",
            "--release",
            "--bin",
            &format!("year{}_day{}", year, day),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("Expected two arguments - year and day");
        return;
    }

    let year = &args[0];
    let day = &args[1];
    let command = if args.len() == 3 { &args[2] } else { "" };

    match command {
        "new" => new_day(year, day),
        "test" => test(year, day),
        "" | "run" => run(year, day),
        _ => eprintln!("Unrecognised command '{}'", command),
    }
}
