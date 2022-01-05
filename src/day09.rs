use regex::Regex;

use lazy_static::lazy_static;
use crate::selfprint::SelfPrint;

fn decompressed_length(string: String, v2: bool) -> Option<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    }

    let mut index = 0;
    let mut len = 0;
    while let Some(m) = RE.find(&string[index..]) {
        len += m.start();
        let captures = RE.captures(m.as_str())?;
        let char_count = captures.get(1)?.as_str().parse::<usize>().ok()?;
        let repeat_count = captures.get(2)?.as_str().parse::<usize>().ok()?;
        if !v2 {
            len += char_count * repeat_count;
        } else {
            len += decompressed_length(
                string[(m.end() + index)..(m.end() + char_count + index)].to_string(),
                true
            ).unwrap_or(0) * repeat_count;
        }
        index += m.end() + char_count;
    }
    len += string.len() - index;
    Some(len)
}

pub fn part1(input: String) {
    decompressed_length(input, false).unwrap_or(0).print();
}

pub fn part2(input: String) {
    decompressed_length(input, true).unwrap_or(0).print();
}

#[cfg(test)]
mod tests {
    use super::decompressed_length;

    #[test]
    fn test_decompressed_length() {
        assert_eq!(decompressed_length("ADVENT".to_string(), false), Some(6));
        assert_eq!(decompressed_length("A(1x5)BC".to_string(), false), Some(7));
        assert_eq!(decompressed_length("(3x3)XYZ".to_string(), false), Some(9));
        assert_eq!(decompressed_length("A(2x2)BCD(2x2)EFG".to_string(), false), Some(11));
        assert_eq!(decompressed_length("(6x1)(1x3)A".to_string(), false), Some(6));
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY".to_string(), false), Some(18));

        assert_eq!(decompressed_length("(3x3)XYZ".to_string(), true), Some(9));
        assert_eq!(decompressed_length("X(8x2)(3x3)ABCY".to_string(), true), Some(20));
        assert_eq!(decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string(), true), Some(241920));
        assert_eq!(decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string(), true), Some(445));
    }
}
