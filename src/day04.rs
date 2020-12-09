use std::collections::HashMap;
use regex::Regex;

const VALID_ECL: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth"
];

pub fn run_a(input: &Vec<String>) -> i64 {
    let passports = Passport::multiple_from(input);

    passports.iter()
        .filter(|p| p.is_valid_basic())
        .count() as i64
}

pub fn run_b(input: &Vec<String>) -> i64 {
    let passports = Passport::multiple_from(input);

    passports.iter()
        .filter(|p| p.is_valid())
        .inspect(|p| debug!("{:?}", p))
        .count() as i64
}

#[derive(Debug,Default)]
struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i32>,
}

impl From<&Vec<String>> for Passport {
    fn from(input: &Vec<String>) -> Self {
        let mut map: HashMap<String, String> = HashMap::new();

        for line in input {
            let pairs = line.split(" ");

            for pair in pairs {
                let mut keyval = pair.split(":");
                let key = keyval.next().unwrap();
                let val = keyval.next().unwrap();
                map.insert(key.into(), val.into());
            }
        }

        Passport {
            byr: map.get("byr").map(|s| s.parse::<i32>().unwrap()),
            iyr: map.get("iyr").map(|s| s.parse::<i32>().unwrap()),
            eyr: map.get("eyr").map(|s| s.parse::<i32>().unwrap()),
            hgt: map.get("hgt").cloned(),
            hcl: map.get("hcl").cloned(),
            ecl: map.get("ecl").cloned(),
            pid: map.get("pid").cloned(),
            cid: map.get("cid").map(|s| s.parse::<i32>().unwrap()),
        }
    }
}

impl Passport {
    pub fn multiple_from(input: &Vec<String>) -> Vec<Passport> {
        let mut lines = vec![];

        let mut passports: Vec<Passport> = vec![];

        for line in input {
            if line == "" {
                passports.push(Passport::from(&lines));
                lines.clear();
            } else {
                lines.push(line.clone());
            }
        }

        if !lines.is_empty() {
            passports.push(Passport::from(&lines));
        }

        passports
    }

    pub fn is_valid_basic(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.is_byr_valid() &&
            self.is_eyr_valid() &&
            self.is_iyr_valid() &&
            self.is_hgt_valid() &&
            self.is_hcl_valid() &&
            self.is_ecl_valid() &&
            self.is_pid_valid()
    }

    pub fn is_byr_valid(&self) -> bool {
        if let Some(y) = self.byr {
            y >= 1920 && y <= 2002
        } else {
            false
        }
    }

    pub fn is_iyr_valid(&self) -> bool {
        if let Some(y) = self.iyr {
            y >= 2010 && y <= 2020
        } else {
            false
        }
    }

    pub fn is_eyr_valid(&self) -> bool {
        if let Some(y) = self.eyr {
            y >= 2020 && y <= 2030
        } else {
            false
        }
    }

    pub fn is_hgt_valid(&self) -> bool {
        if let Some(hgt) = &self.hgt {
            let pattern = Regex::new(r"(\d{2,3})(cm|in)").unwrap();

            if pattern.is_match(&hgt) {
                let captures = pattern.captures(&hgt).unwrap();
                let val = captures.get(1).unwrap().as_str().parse::<i32>();

                if let Ok(v) = val {
                    let unit = captures.get(2).unwrap();

                    if unit.as_str() == "in" {
                        v >= 59 && v <= 76
                    } else if unit.as_str() == "cm" {
                        v >= 150 && v <= 193
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn is_hcl_valid(&self) -> bool {
        if let Some(hcl) = &self.hcl {
            let pattern = Regex::new(r"^#[a-f0-9]{6}$").unwrap();

            pattern.is_match(hcl)
        } else {
            false
        }
    }

    pub fn is_ecl_valid(&self) -> bool {
        if let Some(ecl) = &self.ecl {
            VALID_ECL.contains(&ecl.as_str())
        } else {
            false
        }
    }

    pub fn is_pid_valid(&self) -> bool {
        if let Some(pid) = &self.pid {
            let pattern = Regex::new(r"^[0-9]{9}$").unwrap();

            pattern.is_match(pid)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sample() -> Vec<String> {
        vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        ]
    }

    #[test]
    pub fn passport_parse_all() {
        let _ = env_logger::builder().is_test(true).try_init();

        let input = get_sample();

        let piece: Vec<String> = input[0..2].iter().cloned().collect();

        let passport = Passport::from(&piece);

        assert_eq!("860033327", passport.pid.unwrap());
    }

    #[test]
    pub fn passport_byr_validity() {
        let _ = env_logger::builder().is_test(true).try_init();

        let samples = vec![(2002, true), (2003, false)];

        for (year, expected) in samples {
            let passport = Passport {
                byr: Some(year),
                ..Default::default()
            };

            assert_eq!(expected, passport.is_byr_valid());
        }
    }

    #[test]
    pub fn passport_hgt_validity() {
        let _ = env_logger::builder().is_test(true).try_init();

        let samples = vec![("60in", true), ("190cm", true), ("190in", false), ("190", false)];

        for (val, expected) in samples {
            let passport = Passport {
                hgt: Some(val.to_string()),
                ..Default::default()
            };

            assert_eq!(expected, passport.is_hgt_valid());
        }
    }

    #[test]
    pub fn passport_hcl_validity() {
        let _ = env_logger::builder().is_test(true).try_init();

        let samples = vec![("#123abc", true), ("#123abz", false), ("123abc", false), ];

        for (val, expected) in samples {
            let passport = Passport {
                hcl: Some(val.to_string()),
                ..Default::default()
            };

            assert_eq!(expected, passport.is_hcl_valid());
        }
    }

    #[test]
    pub fn passport_ecl_validity() {
        let _ = env_logger::builder().is_test(true).try_init();

        let samples = vec![("brn", true), ("wat", false)];

        for (val, expected) in samples {
            let passport = Passport {
                ecl: Some(val.to_string()),
                ..Default::default()
            };

            assert_eq!(expected, passport.is_ecl_valid());
        }
    }

    #[test]
    pub fn passport_pid_validity() {
        let _ = env_logger::builder().is_test(true).try_init();

        let samples = vec![("000000001", true), ("0123456789", false)];

        for (val, expected) in samples {
            let passport = Passport {
                pid: Some(val.to_string()),
                ..Default::default()
            };

            assert_eq!(expected, passport.is_pid_valid());
        }
    }

    #[test]
    pub fn passport_invalid() {
        let input = vec![
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
        ];

        let passports = Passport::multiple_from(&input);

        for passport in passports {
            assert_eq!(false, passport.is_valid());
        }
    }

    #[test]
    pub fn passport_valid() {
        let input = vec![
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        ];

        let passports = Passport::multiple_from(&input);

        for passport in passports {
            assert_eq!(true, passport.is_valid());
        }
    }

    #[test]
    pub fn sample_input_0_a() {
        let input = get_sample();

        assert_eq!(2, run_a(&input));
    }
}