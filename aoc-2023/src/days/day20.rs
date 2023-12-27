use std::collections::{HashMap, VecDeque};

use num::{integer::lcm, Integer};

#[derive(PartialEq, Eq, Debug, Clone)]
enum OnOff {
    Off = 0,
    On = 1,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PulseSignal {
    Low = 0,
    High = 1,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

trait Pulse {
    fn pulse(&mut self, signal: PulseSignal, origin: String) -> PulseSignal;
}

impl Pulse for FlipFlop {
    fn pulse(&mut self, signal: PulseSignal, _origin: String) -> PulseSignal {
        match signal {
            PulseSignal::High => signal,
            PulseSignal::Low => match self.on_or_off {
                OnOff::Off => {
                    self.on_or_off = OnOff::On;
                    PulseSignal::High
                }
                OnOff::On => {
                    self.on_or_off = OnOff::Off;
                    PulseSignal::Low
                }
            },
        }
    }
}
#[derive(Debug, Clone)]
struct FlipFlop {
    id: String,
    on_or_off: OnOff,
    destination: Vec<String>,
}

impl Pulse for Conjunction {
    fn pulse(&mut self, signal: PulseSignal, origin: String) -> PulseSignal {
        *(self.input_mem.get_mut(&origin).unwrap()) = signal;

        if self.input_mem.iter().all(|(_k, v)| *v == PulseSignal::High) {
            return PulseSignal::Low;
        }
        PulseSignal::High
    }
}
#[derive(Debug, Clone)]
struct Conjunction {
    id: String,
    input_mem: HashMap<String, PulseSignal>,
    destination: Vec<String>,
}

impl Pulse for Broadcaster {
    fn pulse(&mut self, signal: PulseSignal, _origin: String) -> PulseSignal {
        signal
    }
}
#[derive(Debug, Clone)]
struct Broadcaster {
    id: String,
    destination: Vec<String>,
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut all = HashMap::new();
    input.lines().for_each(|line| {
        let (origin, dests) = line.split_once("->").unwrap();
        let dests = dests
            .trim()
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let origin = origin.trim();

        let tipo = origin.chars().next().unwrap();
        match tipo {
            '%' => {
                let id = origin.chars().skip(1).collect::<String>();
                all.insert(
                    id.clone(),
                    Module::FlipFlop(FlipFlop {
                        id,
                        on_or_off: OnOff::Off,
                        destination: dests,
                    }),
                );
            }
            '&' => {
                let id = origin.chars().skip(1).collect::<String>();
                all.insert(
                    id.clone(),
                    Module::Conjunction(Conjunction {
                        id,
                        input_mem: HashMap::new(),
                        destination: dests,
                    }),
                );
            }
            _ => {
                let id = origin.to_string();
                all.insert(
                    id.clone(),
                    Module::Broadcaster(Broadcaster {
                        id,
                        destination: dests,
                    }),
                );
            }
        };
    });

    for (k, v) in all.clone() {
        match v {
            Module::FlipFlop(f) => {
                for dest in f.destination.iter() {
                    all.entry(dest.clone()).and_modify(|modu| {
                        if let Module::Conjunction(con) = modu {
                            con.input_mem.insert(k.clone(), PulseSignal::Low);
                        }
                    });
                }
            }
            Module::Conjunction(c) => {
                for dest in c.destination.iter() {
                    all.entry(dest.clone()).and_modify(|modu| {
                        if let Module::Conjunction(con) = modu {
                            con.input_mem.insert(k.clone(), PulseSignal::Low);
                        }
                    });
                }
            }
            Module::Broadcaster(b) => {
                for dest in b.destination.iter() {
                    all.entry(dest.clone()).and_modify(|modu| {
                        if let Module::Conjunction(con) = modu {
                            con.input_mem.insert(k.clone(), PulseSignal::Low);
                        }
                    });
                }
            }
        }
    }
    all
}

fn walk(all: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut q = VecDeque::new();
    q.push_back((
        "broadcaster".to_string(),
        PulseSignal::Low,
        "button".to_string(),
    ));

    let mut count_low = 0;
    let mut count_high = 0;

    while let Some((curr, signal, prev)) = q.pop_front() {
        // println!("{} -{:?}-> {}", prev, signal, curr);
        match signal {
            PulseSignal::High => count_high += 1,
            PulseSignal::Low => count_low += 1,
        }
        if !all.contains_key(&curr) {
            continue;
        }
        let curr = all.get_mut(&curr).unwrap();
        match curr {
            Module::Broadcaster(b) => {
                for destiny in b.destination.clone() {
                    q.push_back((destiny, b.pulse(signal.clone(), prev.clone()), b.id.clone()));
                }
            }
            Module::FlipFlop(f) => {
                if signal == PulseSignal::Low {
                    let pulse = f.pulse(signal.clone(), prev.clone());
                    for destiny in f.destination.clone() {
                        q.push_back((destiny, pulse.clone(), f.id.clone()));
                    }
                }
            }
            Module::Conjunction(c) => {
                for destiny in c.destination.clone() {
                    q.push_back((destiny, c.pulse(signal.clone(), prev.clone()), c.id.clone()));
                }
            }
        };

        // all.iter().for_each(|x| println!("{:?}", x));
    }
    (count_low, count_high)
}

fn walk2(all: &mut HashMap<String, Module>) -> Vec<(String, i32)> {
    let count = match all.get("gf").unwrap() {
        Module::Conjunction(con) => con.input_mem.clone(),
        _ => panic!("asd"),
    };
    let mut count = count
        .iter()
        .map(|(id, _)| (id.clone(), 0))
        .collect::<Vec<_>>();

    let mut global_count = 0;
    'lp: loop {
        let mut q = VecDeque::new();

        q.push_back((
            "broadcaster".to_string(),
            PulseSignal::Low,
            "button".to_string(),
        ));

        // for le in count.iter() {
        //     println!("{:?},{global_count}", le);
        // }
        global_count += 1;

        while let Some((curr, signal, prev)) = q.pop_front() {
            // println!("{} -{:?}-> {}", prev, signal, curr);

            if let Some(pos) = count.iter().position(|x| x.0 == prev) {
                if signal == PulseSignal::High && curr == "gf" && count[pos].1 <= 0 {
                    count[pos].1 = global_count;
                }
            }
            if count.iter().all(|x| x.1 > 0) {
                break 'lp;
            }

            match signal {
                PulseSignal::High => {}
                PulseSignal::Low => {}
            }
            if !all.contains_key(&curr) {
                continue;
            }
            let curr = all.get_mut(&curr).unwrap();
            match curr {
                Module::Broadcaster(b) => {
                    for destiny in b.destination.clone() {
                        q.push_back((destiny, b.pulse(signal.clone(), prev.clone()), b.id.clone()));
                    }
                }
                Module::FlipFlop(f) => {
                    if signal == PulseSignal::Low {
                        let pulse = f.pulse(signal.clone(), prev.clone());
                        for destiny in f.destination.clone() {
                            q.push_back((destiny, pulse.clone(), f.id.clone()));
                        }
                    }
                }
                Module::Conjunction(c) => {
                    for destiny in c.destination.clone() {
                        q.push_back((destiny, c.pulse(signal.clone(), prev.clone()), c.id.clone()));
                    }
                }
            };
        }
    }

    count
}
pub fn part_1(input: &str) {
    let mut all = parse(input);
    let mut count = (0, 0);
    let mut repeat = 0;

    for _ in 0..1000 {
        let curr = walk(&mut all);

        count.0 += curr.0;
        count.1 += curr.1;
    }
    println!("{:?}", count.0 * count.1);
    // loop {
    //     let curr = walk(&mut all);
    //     repeat += 1;

    //     count.0 += curr.0;
    //     count.1 += curr.1;

    //     if all.iter().all(|(_k, v)| match v {
    //         Module::Broadcaster(_) => true,
    //         Module::FlipFlop(f) => f.on_or_off == OnOff::Off,
    //         Module::Conjunction(c) => c.input_mem.iter().all(|i| *i.1 == PulseSignal::Low),
    //     }) {
    //         break;
    //     }
    // }

    // println!("\n {:?} repeat: {}", count, repeat);
    // println!()
}

pub fn part_2(input: &str) {
    let mut all = parse(input);

    // all.iter().for_each(|x| println!("{:?}", x));

    let count = walk2(&mut all);

    let ans = count.iter().fold(1, |acc, c| (c.1 as i64).lcm(&acc));
    println!("{}", ans);
}
