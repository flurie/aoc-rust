advent_of_code::solution!(3);

#[derive(Debug, PartialEq, Eq, Clone)]
struct Part {
    y: u32,
    x_start: u32,
    x_end: u32,
    val: u32,
}

impl Part {
    fn coords(&self) -> Vec<(u32, u32)> {
        (self.x_start..self.x_end + 1)
            .into_iter()
            .map(|x| (x, self.y))
            .collect()
    }
}

#[derive(Debug)]
struct Parts(Vec<Part>);

impl Parts {
    fn from_string(s: &str) -> Parts {
        let mut parts: Vec<Part> = vec![];
        s.lines().into_iter().enumerate().for_each(|(y, l)| {
            let b = [l.as_bytes(), &[b'.', b'.', b'.']].concat();
            b.windows(3).enumerate().for_each(|(x, w)| {
                let x_val = x.try_into().unwrap();
                let y_val: u32 = y.try_into().unwrap();
                let (last_x_end, last_y) = match parts.last() {
                    Some(p) => (p.x_end, p.y),
                    None => (0, 0),
                };
                let part = match w {
                    &[i, j, k]
                        if i.is_ascii_digit() && j.is_ascii_digit() && k.is_ascii_digit() =>
                    {
                        Some(Part {
                            y: y.try_into().unwrap_or(0),
                            x_start: x_val,
                            x_end: x_val + 2,
                            val: String::from_utf8_lossy(w).parse::<u32>().unwrap_or(0),
                        })
                    }
                    &[i, j, _]
                        if i.is_ascii_digit()
                            && j.is_ascii_digit()
                            && (x_val > last_x_end || y_val > last_y) =>
                    {
                        Some(Part {
                            y: y.try_into().unwrap_or(0),
                            x_start: x_val,
                            x_end: x_val + 1,
                            val: String::from_utf8_lossy(&w[..2]).parse::<u32>().unwrap_or(0),
                        })
                    }
                    &[i, _, _] if i.is_ascii_digit() && (x_val > last_x_end || y_val > last_y) => {
                        Some(Part {
                            y: y.try_into().unwrap_or(0),
                            x_start: x_val,
                            x_end: x_val,
                            val: String::from_utf8_lossy(&w[..1]).parse::<u32>().unwrap_or(0),
                        })
                    }
                    _ => None,
                };
                if part.is_some() {
                    parts.push(part.unwrap());
                };
            })
        });
        Parts(parts)
    }

    fn at(&self, x: u32, y: u32) -> Option<Part> {
        let mut result: Option<Part> = None;
        for part in self.0.as_slice() {
            if part.coords().contains(&(x, y)) {
                result = Some(part.clone());
                break;
            }
        }
        result
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Kind {
    Space,
    PartNum,
    Part(Part),
    Symbol,
    Gear,
}

struct Elem {
    kind: Kind,
}

impl Elem {
    fn from_byte(b: &u8) -> Elem {
        let kind = match b {
            _ if b == &b'*' => Kind::Gear,
            _ if !b.is_ascii_digit() && !(b == &b'.') => Kind::Symbol,
            _ if b.is_ascii_digit() => Kind::PartNum,
            _ => Kind::Space,
        };
        Elem { kind }
    }
    fn from_byte_with_parts(b: &u8, x: u32, y: u32, p: &Parts) -> Elem {
        let kind = match b {
            _ if b == &b'*' => Kind::Gear,
            _ if !b.is_ascii_digit() && !(b == &b'.') => Kind::Symbol,
            _ if b.is_ascii_digit() => Kind::Part(p.at(x, y).unwrap()),
            _ => Kind::Space,
        };
        Elem { kind }
    }
}

struct Schematic(Vec<Vec<Elem>>);

impl Schematic {
    fn from_string(s: &str) -> Schematic {
        let f = s
            .lines()
            .into_iter()
            .map(|l| {
                l.as_bytes()
                    .iter()
                    .map(|b| Elem::from_byte(b))
                    .collect::<Vec<Elem>>()
            })
            .collect();
        Schematic(f)
    }

    fn from_string_with_parts(s: &str, p: &Parts) -> Schematic {
        let f = s
            .lines()
            .into_iter()
            .enumerate()
            .map(|(y, l)| {
                l.as_bytes()
                    .iter()
                    .enumerate()
                    .map(|(x, b)| {
                        // println!("at {},{}: {}", x, y, b);
                        Elem::from_byte_with_parts(
                            b,
                            x.try_into().unwrap(),
                            y.try_into().unwrap(),
                            p,
                        )
                    })
                    .collect::<Vec<Elem>>()
            })
            .collect();
        Schematic(f)
    }

    fn neighbors(&self, x: u32, y: u32) -> Vec<&Elem> {
        let ix = x as i32;
        let iy = y as i32;
        let cross = (iy - 1..iy + 2)
            .flat_map(|ys| {
                (ix - 1..ix + 2)
                    .clone()
                    .map(move |xs| (xs.clone(), ys.clone()))
            })
            .collect::<Vec<_>>();
        cross
            .into_iter()
            .filter(|(xs, ys)| xs >= &0 && ys >= &0)
            .into_iter()
            .filter_map(|(xs, ys)| match self.0.get(ys as usize) {
                Some(e) => e.get(xs as usize),
                None => None,
            })
            .collect()
    }

    fn neighbor_coords(&self, x: u32, y: u32) -> Vec<(u32, u32)> {
        let ix = x as i32;
        let iy = y as i32;
        (iy - 1..iy + 2)
            .flat_map(|ys| {
                (ix - 1..ix + 2)
                    .clone()
                    .map(move |xs| (xs.clone(), ys.clone()))
            })
            .filter(|(xs, ys)| xs >= &0 && ys >= &0)
            .map(|(x, y)| (x as u32, y as u32))
            .collect::<Vec<_>>()
    }

    fn sym_neighbors(&self, x: u32, y: u32) -> bool {
        let neighbors = self.neighbors(x, y);
        neighbors.into_iter().any(|n| n.kind == Kind::Symbol)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_string(input);
    let parts = Parts::from_string(input);
    let soln = parts
        .0
        .iter()
        .filter_map(|part| {
            if part
                .coords()
                .into_iter()
                .any(|(x, y)| schematic.sym_neighbors(x, y))
            {
                Some(part.val)
            } else {
                Some(0)
            }
        })
        .sum();
    Some(soln)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts = Parts::from_string(input);
    let schematic = Schematic::from_string_with_parts(input, &parts);
    let soln = schematic
        .0
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .map(|(x, e)| match e.kind {
                    Kind::Gear => {
                        let mut part_neighbors = schematic
                            .neighbor_coords(x as u32, y as u32)
                            .iter()
                            .filter_map(|(nx, ny)| parts.at(*nx, *ny))
                            .map(|p| p.val)
                            .collect::<Vec<_>>();
                        part_neighbors.sort();
                        part_neighbors.dedup();
                        match part_neighbors[..] {
                            [p1, p2] => p1 * p2,
                            _ => 0,
                        }
                    }
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum();
    Some(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
