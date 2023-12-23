use phf::phf_map;
use std::io;

fn sum<R: io::BufRead>(reader: R) -> io::Result<i32> {
    let mut sum = 0;
    for l in reader.lines() {
        sum += calibration_value(&l?);
    }

    Ok(sum)
}

static DIGIT_MAP: phf::Map<&'static str, char> = phf_map! {
    "zero"  => '0',
    "one"   => '1',
    "two"   => '2',
    "three" => '3',
    "four"  => '4',
    "five"  => '5',
    "six"   => '6',
    "seven" => '7',
    "eight" => '8',
    "nine"  => '9',
};

fn calibration_value(line: &str) -> i32 {
    let (first, last) = {
        let mut first = '0';
        'outer: for (i, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                first = c;
                break;
            }

            let rest = &line[i..];
            for (w, c) in DIGIT_MAP.into_iter() {
                if rest.starts_with(w) {
                    first = *c;
                    break 'outer;
                }
            }
        }

        let mut last = '0';
        let rev: String = line.chars().rev().collect();
        'outer: for (i, c) in rev.chars().enumerate() {
            if c.is_ascii_digit() {
                last = c;
                break;
            }

            let rest = &rev[i..];
            for (w, c) in DIGIT_MAP.into_iter() {
                if rest.starts_with(&w.chars().rev().collect::<String>()) {
                    last = *c;
                    break 'outer;
                }
            }
        }

        (first, last)
    };

    format!("{}{}", first, last).parse().unwrap_or(0)
}

fn main() -> io::Result<()> {
    let answer = sum(io::stdin().lock())?;
    println!("{}", answer);

    Ok(())
}
