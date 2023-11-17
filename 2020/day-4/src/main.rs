use std::fs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, multispace0, none_of, space1};
use nom::combinator::{eof, map, recognize};
use nom::IResult;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::tuple;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let filename = "input.txt";
    let input = fs::read_to_string(&filename)?;
    let result = part1(&input)?;
    println!("{result}");
    let result = part2(&input)?;
    println!("{result}");
    Ok(())
}

fn part1(input: &str) -> anyhow::Result<usize> {
    let (_, passports) = passport_file(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(passports.iter().filter(|passport| passport.is_valid_part1()).count())
}

fn part2(input: &str) -> anyhow::Result<usize> {
    let (_, passports) = passport_file(input).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(passports.iter().filter(|passport| passport.is_valid_part2()).count())
}

#[derive(Debug, PartialEq, Eq)]
enum PasswordField {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

#[derive(Debug)]
struct Passport {
    fields: Vec<KeyValue>,
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.contains_field(&PasswordField::BYR) &&
            self.contains_field(&PasswordField::IYR) &&
            self.contains_field(&PasswordField::EYR) &&
            self.contains_field(&PasswordField::HGT) &&
            self.contains_field(&PasswordField::HCL) &&
            self.contains_field(&PasswordField::ECL) &&
            self.contains_field(&PasswordField::PID)
        // ignore CID
    }

    fn is_valid_part2(&self) -> bool {
        self.is_valid_part1() && self.fields.iter().all(|kv| kv.is_valid())
    }

    fn contains_field(&self, field: &PasswordField) -> bool {
        self.fields.iter().any(|kv| &kv.key == field)
    }
}

#[derive(Debug)]
struct KeyValue {
    key: PasswordField,
    value: String,
}

impl KeyValue {
    fn is_valid(&self) -> bool {
        static HGT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap());
        static PID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\d{9}$"#).unwrap());

        match self.key {
            PasswordField::BYR => {
                let year = self.value.parse::<u32>().unwrap_or(0);
                1920 <= year && year <= 2002
            }
            PasswordField::IYR => {
                let year = self.value.parse::<u32>().unwrap_or(0);
                2010 <= year && year <= 2020
            }
            PasswordField::EYR => {
                let year = self.value.parse::<u32>().unwrap_or(0);
                2020 <= year && year <= 2030
            }
            PasswordField::HGT => {
                let is_cm = self.value.ends_with("cm");
                let is_in = self.value.ends_with("in");
                let mut chars = self.value.chars();
                chars.next_back();
                chars.next_back();
                let input = chars.as_str();
                let height = input.parse::<u32>().unwrap_or(0);
                if is_cm {
                    150 <= height && height <= 193
                } else if is_in {
                    59 <= height && height <= 76
                } else {
                    false
                }
            }
            PasswordField::HCL => {
                HGT_RE.is_match(&self.value)
            }
            PasswordField::ECL => {
                let color = self.value.as_str();
                color == "amb" || color == "blu" || color == "brn" || color == "gry" || color == "grn" || color == "hzl" || color == "oth"
            }
            PasswordField::PID => PID_RE.is_match(&self.value),
            PasswordField::CID => true,
        }
    }
}

fn passport_file(input: &str) -> IResult<&str, Vec<Passport>> {
    let (input, passports) = passports(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, passports))
}

fn passports(input: &str) -> IResult<&str, Vec<Passport>> {
    separated_list0(tuple((line_ending, line_ending)), passport)(input)
}

fn passport(input: &str) -> IResult<&str, Passport> {
    let (input, fields) = separated_list1(alt((space1, line_ending)), passport_kv)(input)?;
    Ok((input, Passport { fields }))
}

fn passport_kv(input: &str) -> IResult<&str, KeyValue> {
    let (input, key) = password_field(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = recognize(many1(none_of(" \n")))(input)?;
    Ok((input, KeyValue { key, value: value.to_string() }))
}

fn password_field(input: &str) -> IResult<&str, PasswordField> {
    let byr = map(tag("byr"), |_| PasswordField::BYR);
    let iyr = map(tag("iyr"), |_| PasswordField::IYR);
    let eyr = map(tag("eyr"), |_| PasswordField::EYR);
    let hgt = map(tag("hgt"), |_| PasswordField::HGT);
    let hcl = map(tag("hcl"), |_| PasswordField::HCL);
    let ecl = map(tag("ecl"), |_| PasswordField::ECL);
    let pid = map(tag("pid"), |_| PasswordField::PID);
    let cid = map(tag("cid"), |_| PasswordField::CID);
    alt((byr,
         iyr,
         eyr,
         hgt,
         hcl,
         ecl,
         pid,
         cid, ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let result = part1(INPUT)?;
        assert_eq!(2, result);
        Ok(())
    }

    static INVALID: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

    static VALID: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;

    #[test]
    fn part2_invalid_passports() -> anyhow::Result<()> {
        let count = part2(&INVALID)?;
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn part2_valid_passports() -> anyhow::Result<()> {
        let count = part2(&VALID)?;
        let expected = 4;
        assert_eq!(count, expected);
        Ok(())
    }
}