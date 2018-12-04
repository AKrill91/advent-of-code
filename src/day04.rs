use std::collections::HashMap;

use chrono::DateTime;
use chrono::Timelike;
use chrono::TimeZone;
use chrono::Utc;
use regex::Captures;
use regex::Regex;

struct GuardShift {
    guard_id: i32,
    shift_start: DateTime<Utc>,
}

struct GuardNap {
    guard_id: i32,
    nap_start: DateTime<Utc>,
    nap_end: DateTime<Utc>,
}

pub fn run_a(input: &Vec<String>) -> i32 {
    let naps = parse_shifts(input);
    let num_naps = naps.len();

    info!("Found {} naps", num_naps);

    let mut guard_nap_map = HashMap::new();
    let mut guard_nap_schedule = HashMap::new();

    for nap in naps {
        let count = guard_nap_map.entry(nap.guard_id).or_insert(0);
        *count += nap.nap_end.signed_duration_since(nap.nap_start).num_minutes();

        let start_min = nap.nap_start.minute();
        let end_min = nap.nap_end.minute();

        for i in start_min..end_min {
            let map = guard_nap_schedule.entry(nap.guard_id).or_insert(HashMap::new());
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
    }

    let mut sleepiest_guard_id = -1;
    let mut sleepiest_guard_nap_time = 0;

    for (guard_id, nap_time) in guard_nap_map {
        if nap_time > sleepiest_guard_nap_time {
            sleepiest_guard_id = guard_id;
            sleepiest_guard_nap_time = nap_time;
        }
    }

    let mut sleepiest_minute: u32 = 0;
    let mut sleepiest_minute_count = -1;

    let sleepiest_guard_schedule = guard_nap_schedule.get(&sleepiest_guard_id).unwrap();
    for (minute, nap_count) in sleepiest_guard_schedule.iter() {
        if *nap_count > sleepiest_minute_count {
            sleepiest_minute = *minute;
            sleepiest_minute_count = *nap_count;
        }
    }

    info!("Guard {} is the sleepiest guard, had {} naps at minute {}", sleepiest_guard_id, sleepiest_minute_count, sleepiest_minute);

    let result = sleepiest_guard_id * sleepiest_minute as i32;

    info!("Result is {}", result);

    result
}

pub fn run_b(input: &Vec<String>) -> i32 {
    let naps = parse_shifts(input);
    let num_naps = naps.len();

    info!("Found {} naps", num_naps);

    let mut guard_nap_map = HashMap::new();
    let mut guard_nap_schedule = HashMap::new();

    for nap in naps {
        let count = guard_nap_map.entry(nap.guard_id).or_insert(0);
        *count += nap.nap_end.signed_duration_since(nap.nap_start).num_minutes();

        let start_min = nap.nap_start.minute();
        let end_min = nap.nap_end.minute();

        for i in start_min..end_min {
            let map = guard_nap_schedule.entry(nap.guard_id).or_insert(HashMap::new());
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
    }

    let mut sleepiest_guard_id = -1;
    let mut sleepiest_minute = 0;
    let mut sleepiest_minute_count = -1;

    for (guard_id, schedule) in guard_nap_schedule.iter() {
        for (minute, nap_count) in schedule.iter() {
            if *nap_count > sleepiest_minute_count {
                sleepiest_guard_id = *guard_id;
                sleepiest_minute = *minute;
                sleepiest_minute_count = *nap_count;
            }
        }
    }

    info!("Guard {} is the sleepiest guard, had {} naps at minute {}", sleepiest_guard_id, sleepiest_minute_count, sleepiest_minute);

    let result = sleepiest_guard_id * sleepiest_minute as i32;

    info!("Result is {}", result);

    result
}

fn parse_shifts(input: &Vec<String>) -> Vec<GuardNap> {
    let mut shifts: Vec<GuardShift> = Vec::new();
    let mut sleep_starts: Vec<DateTime<Utc>> = Vec::new();
    let mut sleep_ends: Vec<DateTime<Utc>> = Vec::new();
    let mut naps: Vec<GuardNap> = Vec::new();

    let shift_start_pattern = Regex::new(r"\[1518-(\d\d)-(\d\d) (\d\d):(\d\d)] Guard #(\d+) begins shift").unwrap();
    let sleep_start_pattern = Regex::new(r"\[1518-(\d\d)-(\d\d) (\d\d):(\d\d)] falls asleep").unwrap();
    let sleep_end_pattern = Regex::new(r"\[1518-(\d\d)-(\d\d) (\d\d):(\d\d)] wakes up").unwrap();

    for line in input {
        if shift_start_pattern.is_match(line) {
            let captures = shift_start_pattern.captures(line).unwrap();
            let guard_id = captures.get(5).unwrap().as_str().parse::<i32>().unwrap();

            let shift_start: DateTime<Utc> = date_from_captures(&captures);

            shifts.push(GuardShift { guard_id, shift_start })
        } else if sleep_start_pattern.is_match(line) {
            let captures = sleep_start_pattern.captures(line).unwrap();

            let sleep_start = date_from_captures(&captures);
            sleep_starts.push(sleep_start);
        } else if sleep_end_pattern.is_match(line) {
            let captures = sleep_end_pattern.captures(line).unwrap();

            let sleep_end = date_from_captures(&captures);
            sleep_ends.push(sleep_end);
        } else {
            panic!("Unable to parse line: {}", line);
        }
    }

    shifts.sort_by_key(|shift| shift.shift_start);
    shifts.reverse();

    sleep_starts.sort();
    sleep_ends.sort();

    let zipped = sleep_starts.iter().zip(sleep_ends.iter());

    for (start, end) in zipped {
        for shift in shifts.iter() {
            if shift.shift_start.lt(start) {
                naps.push(
                    GuardNap {
                        guard_id: shift.guard_id,
                        nap_start: *start,
                        nap_end: *end,
                    }
                );
                break;
            }
        }
    }

    naps
}

fn date_from_captures(captures: &Captures) -> DateTime<Utc> {
    let month = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let date = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let hour = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let minute = captures.get(4).unwrap().as_str().parse::<u32>().unwrap();

    Utc.ymd(1518, month, date).and_hms(hour, minute, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_a() {
        let sample: Vec<String> = vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-01 00:25] wakes up"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up")
        ];

        assert_eq!(240, run_a(&sample));
    }

    #[test]
    fn sample_input_reordered_a() {
        let sample: Vec<String> = vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up"),
            String::from("[1518-11-01 00:25] wakes up")
        ];

        assert_eq!(240, run_a(&sample));
    }

    #[test]
    fn sample_input_b() {
        let sample: Vec<String> = vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up"),
            String::from("[1518-11-01 00:25] wakes up")
        ];

        assert_eq!(4455, run_b(&sample));
    }
}