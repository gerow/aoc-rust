use std::io;

fn sum<R: io::BufRead>(reader: R) -> io::Result<i32> {
    let mut sum = 0;
    for l in reader.lines() {
        sum += calibration_value(&l?);
    }

    Ok(sum)
}

fn calibration_value(line: &str) -> i32 {
    let (first, last) = {
        let mut first = '0';
        for c in line.chars() {
            if c.is_ascii_digit() {
                first = c;
                break;
            }
        }

        let mut last = '0';
        for c in line.chars().rev() {
            if c.is_ascii_digit() {
                last = c;
                break;
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
