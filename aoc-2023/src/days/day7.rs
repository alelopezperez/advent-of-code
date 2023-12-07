use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A = 15,
    K = 14,
    Q = 13,
    J = 12, // change j to 12
    T = 11,
    NINE = 10,
    EIGT = 9,
    SEVEN = 8,
    SIX = 7,
    FIVE = 6,
    FOUR = 5,
    THREE = 4,
    TWO = 3,
    JOKER = 1,
    INVALID = 2,
}

impl HandType {
    fn calculate_type(hand: &[Card; 5]) -> HandType {
        let count = hand
            .iter()
            .fold(Vec::<(&Card, u32)>::new(), |mut accum, curr| {
                if accum.is_empty() {
                    accum.push((curr, 1));
                } else {
                    if let Some(found) = accum.iter_mut().find(|x| x.0 == curr) {
                        found.1 += 1;
                    } else {
                        accum.push((curr, 1));
                    }
                }
                accum
            });

        if count.len() == 1 {
            HandType::FiveOKind
        } else if count.len() == 2 {
            if count[0].1 == 1 || count[1].1 == 1 {
                HandType::FourOKind
            } else {
                HandType::FullHouse
            }
        } else if count.len() == 3 {
            if count[0].1 == 3 || count[1].1 == 3 || count[2].1 == 3 {
                HandType::ThreeOKind
            } else {
                HandType::TwoPair
            }
        } else if count.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn calculate_type_2(hand: &[Card; 5]) -> HandType {
        let mut count = hand
            .iter()
            .fold(Vec::<(&Card, u32)>::new(), |mut accum, curr| {
                if accum.is_empty() {
                    accum.push((curr, 1));
                } else {
                    if let Some(found) = accum.iter_mut().find(|x| x.0 == curr) {
                        found.1 += 1;
                    } else {
                        accum.push((curr, 1));
                    }
                }
                accum
            });
        //NEW LOGIC
        let jokers = count
            .iter()
            .cloned()
            .enumerate()
            .find(|(_i, x)| *x.0 == Card::JOKER);

        match jokers {
            Some(jokes) => {
                if count.len() > 1 {
                    count.remove(jokes.0);

                    let max = count
                        .iter()
                        .cloned()
                        .enumerate()
                        .max_by(|a, b| a.1 .1.cmp(&b.1 .1))
                        .unwrap();

                    count[max.0].1 += jokes.1 .1;
                }
            }
            None => {}
        }
        //NEW LOGIC

        if count.len() == 1 {
            HandType::FiveOKind
        } else if count.len() == 2 {
            if count[0].1 == 1 || count[1].1 == 1 {
                HandType::FourOKind
            } else {
                HandType::FullHouse
            }
        } else if count.len() == 3 {
            if count[0].1 == 3 || count[1].1 == 3 || count[2].1 == 3 {
                HandType::ThreeOKind
            } else {
                HandType::TwoPair
            }
        } else if count.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    FiveOKind = 7,
    FourOKind = 6,
    FullHouse = 5,
    ThreeOKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    fn new(hand: Vec<char>) -> Self {
        let card: [Card; 5] = hand
            .into_iter()
            .map(|card| match card {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::NINE,
                '8' => Card::EIGT,
                '7' => Card::SEVEN,
                '6' => Card::SIX,
                '5' => Card::FIVE,
                '4' => Card::FOUR,
                '3' => Card::THREE,
                '2' => Card::TWO,
                _ => Card::INVALID,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let hand_type = HandType::calculate_type(&card);

        Self {
            hand: card,
            value: hand_type,
        }
    }
    fn new_joker(hand: Vec<char>) -> Self {
        let card: [Card; 5] = hand
            .into_iter()
            .map(|card| match card {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::JOKER,
                'T' => Card::T,
                '9' => Card::NINE,
                '8' => Card::EIGT,
                '7' => Card::SEVEN,
                '6' => Card::SIX,
                '5' => Card::FIVE,
                '4' => Card::FOUR,
                '3' => Card::THREE,
                '2' => Card::TWO,
                _ => Card::INVALID,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let hand_type = HandType::calculate_type_2(&card);

        Self {
            hand: card,
            value: hand_type,
        }
    }
}
#[derive(Debug)]
struct Hand {
    hand: [Card; 5],
    value: HandType,
}

fn parse_1(input: String) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|x| {
            let (hand, bet) = x.split_once(' ').unwrap();
            let hand = hand.chars().collect::<Vec<_>>();
            (Hand::new(hand), bet.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>()
}

fn parse_2(input: String) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|x| {
            let (hand, bet) = x.split_once(' ').unwrap();
            let hand = hand.chars().collect::<Vec<_>>();
            (Hand::new_joker(hand), bet.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>()
}
pub fn part_1(input: String) {
    let mut game = parse_1(input);
    game.sort_by(|b, a| {
        let compare = b.0.value.cmp(&a.0.value);

        if compare == Ordering::Equal {
            b.0.hand
                .iter()
                .zip(a.0.hand.iter())
                .map(|(x, y)| x.cmp(&y))
                .find(|x| x != &Ordering::Equal)
                .unwrap()
        } else {
            compare
        }
    });

    let total = game.iter().enumerate().fold(0, |accum, (index, item)| {
        accum + (item.1 * ((index as u32) + 1))
    });
    println!("{:#?}", total);
}

pub fn part_2(input: String) {
    let mut game = parse_2(input);
    game.sort_by(|b, a| {
        let compare = b.0.value.cmp(&a.0.value);

        if compare == Ordering::Equal {
            b.0.hand
                .iter()
                .zip(a.0.hand.iter())
                .map(|(x, y)| x.cmp(&y))
                .find(|x| x != &Ordering::Equal)
                .unwrap()
        } else {
            compare
        }
    });

    let total = game.iter().enumerate().fold(0, |accum, (index, item)| {
        accum + (item.1 * ((index as u32) + 1))
    });
    println!("{:#?}", total);
}
