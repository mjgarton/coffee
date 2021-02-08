#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]

use rand::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let mut c = CoffeeMeetings::new();

    c.add_participant(Person::new("alice"));
    c.add_participant(Person::new("bob"));
    c.add_participant(Person::new("chris"));
    c.add_participant(Person::new("dave"));
    c.add_participant(Person::new("elsa"));
    c.add_participant(Person::new("fred"));
    c.add_participant(Person::new("george"));
    c.add_participant(Person::new("harriet"));
    c.add_participant(Person::new("imogen"));
    c.add_participant(Person::new("julian"));
    c.add_participant(Person::new("kevin"));
    c.add_participant(Person::new("lisa"));

    for _i in 0..11 {
        let next = c.calculate_next();
    }

    for (i, round) in c.result_history.iter().enumerate() {
        println!("round {}", i + 1);
        for m in &round.meetings {
            let mut p = m.people.iter().collect::<Vec<_>>();
            p.sort();
            p.iter().for_each(|&x| print!("{} ", x.name));

            println!("");
        }
        println!("");
    }
}

struct CoffeeMeetings {
    result_history: Vec<RoundResult>,
    participants: Vec<Person>,
}

impl CoffeeMeetings {
    fn new() -> Self {
        CoffeeMeetings {
            result_history: Vec::new(),
            participants: Vec::new(),
        }
    }

    fn calculate_next(&mut self) -> &RoundResult {
        // pre calculate costs for each pair that might meet.
        let mut costs = HashMap::new();
        for person1 in &self.participants {
            for person2 in &self.participants {
                costs.insert((person1, person2), self.cost(person1, person2));
            }
        }

        let mut rng = rand::thread_rng();

        let mut best = None;
        let mut best_cost = 1_f32;

        for _i in 1..100000 {
            let mut people = self.participants.iter().collect::<Vec<_>>();
            people.shuffle(&mut rng);

            let mut people: &[&Person] = &people;

            let mut rr = RoundResult {
                meetings: Vec::new(),
            };

            while !people.is_empty() {
                match people.len() {
                    3 => {
                        let (three, remaining) = people.split_at(3);
                        rr.meetings.push(Meeting::new(&three));
                        people = remaining;
                    }

                    2.. => {
                        let (two, remaining) = people.split_at(2);
                        rr.meetings.push(Meeting::new(&two));
                        people = remaining;
                    }
                    0 => {}
                    _ => {
                        panic!("bug")
                    }
                };
            }

            let mut round_cost = 0_f32;
            for m in &rr.meetings {
                match m.people.len() {
                    2 => {
                        let mut p = m.people.iter();
                        let p1 = p.next().unwrap();
                        let p2 = p.next().unwrap();
                        round_cost += costs[&(p1, p2)];
                    }
                    3 => {
                        let mut p = m.people.iter();
                        let p1 = p.next().unwrap();
                        let p2 = p.next().unwrap();
                        let p3 = p.next().unwrap();
                        round_cost += costs[&(p1, p2)];
                        round_cost += costs[&(p2, p3)];
                        round_cost += costs[&(p1, p3)];
                    }
                    _ => {
                        panic!("bug")
                    }
                }
            }

            if let None = best {
                best = Some(rr);
                best_cost = round_cost;
            } else {
                if round_cost < best_cost {
                    best_cost = round_cost;
                    best = Some(rr)
                }
            }
        }

        self.result_history.push(best.unwrap());

        self.result_history.last().unwrap()
    }

    // Returns a cost score for two people meeting. 0 means never met before. 1 means met in the most recent round
    fn cost(&self, person1: &Person, person2: &Person) -> f32 {
        for (i, round_result) in self.result_history.iter().rev().enumerate() {
            if round_result.met(person1, person2) {
                return 1_f32 / ((i + 1) as f32);
            }
        }
        0_f32
    }

    fn latest(&self) -> Option<&RoundResult> {
        self.result_history.iter().next()
    }

    fn add_participant(&mut self, p: Person) {
        self.participants.push(p)
    }
}

#[derive(Debug)]
struct Meeting {
    people: HashSet<Person>,
}

impl Meeting {
    fn new(people: &[&Person]) -> Self {
        let mut m = Meeting {
            people: HashSet::new(),
        };
        m.add_participants(people);
        m
    }

    fn add_participants(&mut self, people: &[&Person]) {
        for &p in people {
            self.people.insert(p.clone());
        }
    }
}

#[derive(Debug)]
struct RoundResult {
    meetings: Vec<Meeting>,
}

impl RoundResult {
    // fn met_who(&self, person : &Person) -> Vec<Person> {
    //     for meeting in &self.meetings {
    //         for person2 in &meeting.people {
    //             if person == person2 {
    //                 return meeting.people.iter().filter(|x| *x != person).map(|x| x.clone()).collect()
    //             }
    //         }
    //     }
    //     vec![]
    // }

    fn met(&self, person1: &Person, person2: &Person) -> bool {
        for meeting in &self.meetings {
            if meeting.people.contains(person1) && meeting.people.contains(person2) {
                return true;
            }
        }
        false
    }
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new(name: &str) -> Self {
        Person {
            name: name.to_owned(),
        }
    }
}
