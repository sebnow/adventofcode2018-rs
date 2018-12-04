use chrono::{DateTime, FixedOffset, Timelike};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Event {
    timestamp: DateTime<FixedOffset>,
    kind: EventKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    ShiftChange { id: u32 },
    WakeUp,
    FallAsleep,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Event> {
    let mut events: Vec<Event> = input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(" ").collect();
            let date_part = format!("{} {}+0000", parts[0], parts[1]);

            let timestamp = DateTime::parse_from_str(&date_part, "[%Y-%m-%d %H:%M]%z").unwrap();

            match parts[2] {
                "Guard" => {
                    let (_, id) = parts[3].split_at(1);
                    Event {
                        timestamp: timestamp,
                        kind: EventKind::ShiftChange {
                            id: id.parse::<u32>().unwrap(),
                        },
                    }
                }
                "falls" => Event {
                    timestamp: timestamp,
                    kind: EventKind::FallAsleep,
                },
                "wakes" => Event {
                    timestamp: timestamp,
                    kind: EventKind::WakeUp,
                },
                _ => panic!("unknown event"),
            }
        }).collect();
    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    events
}

#[aoc(day4, part1)]
fn answer_1(events: &[Event]) -> usize {
    let mut guard_sleep: HashMap<u32, u32> = HashMap::with_capacity(events.len());
    let mut guard_minute: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut current_id = 0;
    let mut last_event = &events[0];

    for ev in events {
        match ev.kind {
            EventKind::ShiftChange { id } => {
                current_id = id;
                guard_sleep.entry(current_id).or_insert(0);
            }

            EventKind::WakeUp => {
                let duration = guard_sleep.get_mut(&current_id).unwrap();
                match last_event.kind {
                    EventKind::FallAsleep => {
                        for m in last_event.timestamp.minute()..ev.timestamp.minute() {
                            let minutes = guard_minute.entry(current_id).or_insert_with(|| {
                                let mut v = Vec::with_capacity(60);
                                for _ in 0..60 {
                                    v.push(0);
                                }
                                v
                            });

                            minutes[m as usize] += 1;
                        }
                        let minutes = ev.timestamp - last_event.timestamp;
                        *duration += minutes.num_minutes() as u32;
                    }
                    _ => panic!("Woke up without falling asleep"),
                }
            }

            EventKind::FallAsleep => last_event = ev,
        }
        last_event = ev;
    }

    let (sleepy_id, total_minutes) =
        guard_sleep
            .iter()
            .fold((0, 0), |(c_id, c_asleep), (&id, &asleep)| {
                if asleep > c_asleep {
                    (id, asleep)
                } else {
                    (c_id, c_asleep)
                }
            });

    let sleepy = guard_minute.get(&sleepy_id).unwrap();
    let max = sleepy.iter().max().unwrap();
    let minute = sleepy.iter().position(|x| x == max).unwrap();

    sleepy_id as usize * minute
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_1() {
        let input = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up\
";
        assert_eq!(answer_1(&input_generator(input)), 240);
    }
}
