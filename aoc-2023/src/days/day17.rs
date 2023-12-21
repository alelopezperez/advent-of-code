use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet, VecDeque};

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Node {
    pos: (usize, usize),
    direction: Direction,
    step: usize,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Node,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn dijkstra<F: Fn(&Node, &Vec<Vec<i32>>) -> Vec<Node>>(puzzle: Vec<Vec<i32>>, get_nei: F) -> i32 {
    let mut visited = HashMap::new();
    let mut queue = BinaryHeap::new();

    visited.insert(
        Node {
            pos: (0, 0),
            direction: Direction::Right,
            step: 0,
        },
        0,
    );

    visited.insert(
        Node {
            pos: (0, 0),
            direction: Direction::Down,
            step: 0,
        },
        0,
    );

    queue.push(State {
        node: Node {
            pos: (0, 0),
            direction: Direction::Right,
            step: 0,
        },
        cost: 0,
    });
    queue.push(State {
        node: Node {
            pos: (0, 0),
            direction: Direction::Down,
            step: 0,
        },
        cost: 0,
    });

    while let Some(state) = queue.pop() {
        let node = state.node;
        let cost = state.cost as i32;

        if node.pos == (puzzle.len() - 1, puzzle[0].len() - 1) && node.step >= 4 {
            return cost;
        }

        let all_nei = get_nei(&node, &puzzle);

        for nei in all_nei.iter() {
            let new_cost = cost + puzzle[nei.pos.0][nei.pos.1];

            if let Some(&cost) = visited.get(nei) {
                if new_cost >= cost {
                    continue;
                }
            }
            visited.insert(nei.clone(), new_cost);
            queue.push(State {
                node: nei.clone(),
                cost: new_cost as usize,
            });
        }
    }

    i32::MAX
}

fn get_next(node: &Node, puzzle: &Vec<Vec<i32>>) -> Vec<Node> {
    let mut nei = Vec::new();
    match node.direction {
        Direction::Up => {
            if node.step == 3 {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            } else {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: node.step + 1,
                    });
                }
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            }
        }
        Direction::Down => {
            if node.step == 3 {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            } else {
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: node.step + 1,
                    });
                }
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            }
        }
        Direction::Left => {
            if node.step == 3 {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            } else {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: node.step + 1,
                    });
                }

                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            }
        }
        Direction::Right => {
            if node.step == 3 {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            } else {
                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: node.step + 1,
                    });
                }

                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            }
        }
    }

    nei
}

fn get_next2(node: &Node, puzzle: &Vec<Vec<i32>>) -> Vec<Node> {
    let mut nei = Vec::new();
    match node.direction {
        Direction::Up => {
            if node.step == 10 {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            } else {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: node.step + 1,
                    });
                }
                if node.step >= 4 {
                    if node.pos.1 > 0 {
                        nei.push(Node {
                            pos: (node.pos.0, node.pos.1 - 1),
                            direction: Direction::Left,
                            step: 1,
                        });
                    }

                    if node.pos.1 < puzzle[0].len() - 1 {
                        nei.push(Node {
                            pos: (node.pos.0, node.pos.1 + 1),
                            direction: Direction::Right,
                            step: 1,
                        });
                    }
                }
            }
        }
        Direction::Down => {
            if node.step == 10 {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: 1,
                    });
                }

                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: 1,
                    });
                }
            } else {
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: node.step + 1,
                    });
                }
                if node.step >= 4 {
                    if node.pos.1 > 0 {
                        nei.push(Node {
                            pos: (node.pos.0, node.pos.1 - 1),
                            direction: Direction::Left,
                            step: 1,
                        });
                    }

                    if node.pos.1 < puzzle[0].len() - 1 {
                        nei.push(Node {
                            pos: (node.pos.0, node.pos.1 + 1),
                            direction: Direction::Right,
                            step: 1,
                        });
                    }
                }
            }
        }
        Direction::Left => {
            if node.step == 10 {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            } else {
                if node.pos.1 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 - 1),
                        direction: Direction::Left,
                        step: node.step + 1,
                    });
                }

                if node.step >= 4 {
                    if node.pos.0 > 0 {
                        nei.push(Node {
                            pos: (node.pos.0 - 1, node.pos.1),
                            direction: Direction::Up,
                            step: 1,
                        });
                    }
                    if node.pos.0 < puzzle.len() - 1 {
                        nei.push(Node {
                            pos: (node.pos.0 + 1, node.pos.1),
                            direction: Direction::Down,
                            step: 1,
                        });
                    }
                }
            }
        }
        Direction::Right => {
            if node.step == 10 {
                if node.pos.0 > 0 {
                    nei.push(Node {
                        pos: (node.pos.0 - 1, node.pos.1),
                        direction: Direction::Up,
                        step: 1,
                    });
                }
                if node.pos.0 < puzzle.len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0 + 1, node.pos.1),
                        direction: Direction::Down,
                        step: 1,
                    });
                }
            } else {
                if node.pos.1 < puzzle[0].len() - 1 {
                    nei.push(Node {
                        pos: (node.pos.0, node.pos.1 + 1),
                        direction: Direction::Right,
                        step: node.step + 1,
                    });
                }

                if node.step >= 4 {
                    if node.pos.0 > 0 {
                        nei.push(Node {
                            pos: (node.pos.0 - 1, node.pos.1),
                            direction: Direction::Up,
                            step: 1,
                        });
                    }
                    if node.pos.0 < puzzle.len() - 1 {
                        nei.push(Node {
                            pos: (node.pos.0 + 1, node.pos.1),
                            direction: Direction::Down,
                            step: 1,
                        });
                    }
                }
            }
        }
    }

    nei
}

pub fn part_1(input: &str) {
    let puzzle = parse(input);

    let ans = dijkstra(puzzle, get_next);
    println!("{ans}")
}

pub fn part_2(input: &str) {
    let puzzle = parse(input);

    let ans = dijkstra(puzzle, get_next2);
    println!("{ans}")
}
