use chrono::{DateTime, Utc, FixedOffset};

// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn total(data: Vec<(DateTime<FixedOffset>, i32)>) -> i64 {
    dbg!(&"xxxx");
    // use iter to sum all the values
    let result: Vec<_> = data
        .iter()
        .zip(data.iter().skip(1))
        .inspect(|x| println!("{:?}", x))
        .map(|(x, y)| {
            // trapezoidal rule
            let (time1, value1) = x;
            let (time2, value2) = y;
            let time_diff = time2.signed_duration_since(*time1).num_microseconds().unwrap() as i64;
            let value_diff = value2 - value1;
            let area = ((value1 + value2) as i64).checked_mul(time_diff).unwrap() / 2;
            area as i64
        })
        .inspect(|x| println!("vvv: {:?}", x))
        .collect();
    dbg!(&result);
    result.iter().sum::<i64>().abs()
}

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the parent module

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5); // Test case 1
        assert_eq!(add(-1, 1), 0); // Test case 2
        assert_eq!(add(0, 0), 0); // Test case 3
        assert_eq!(add(1, 1), 2); // Test case 3
    }

    #[test]
    fn test_total(){
        // array with timestamp and value
        let data = vec![(DateTime::parse_from_rfc3339("2025-01-24T03:15:17.400946+00:00").unwrap(), 100), (DateTime::parse_from_rfc3339("2025-01-24T03:14:15.335619+00:00").unwrap(), 200), (DateTime::parse_from_rfc3339("2025-01-24T03:13:13.321782+00:00").unwrap(), 300)];
        assert_eq!(total(data), 24813258300);
    }
}



fn main() {
    println!("Hello, world!");
}
