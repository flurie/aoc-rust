use std::{collections::HashMap, ops::Range, str::FromStr};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Seeds(Vec<u64>);

struct ParseSeedsError;

impl FromStr for Seeds {
    type Err = ParseSeedsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds = s
            .strip_prefix("seeds: ")
            .ok_or(ParseSeedsError)?
            .split(" ")
            .map(|s| s.parse().ok().unwrap())
            .collect::<Vec<_>>();
        Ok(Seeds(seeds))
    }
}

struct SeedPairs(Vec<u64>);

impl FromStr for SeedPairs {
    type Err = ParseSeedsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let seeds: Vec<u64> = s
            .strip_prefix("seeds: ")
            .ok_or(ParseSeedsError)?
            .split(" ")
            .map(|s| s.parse().ok().unwrap())
            .collect::<Vec<_>>();
        let pairs = seeds
            .chunks_exact(2)
            .map(|c| match c.len() {
                2 => (c[0]..c[0] + c[1]).collect::<Vec<_>>(),
                _ => vec![],
            })
            .collect::<Vec<_>>()
            .concat();
        Ok(SeedPairs(pairs))
    }
}

struct ParseTranslatorError;

#[derive(Debug)]
struct Translator(HashMap<Range<u64>, i64>);

impl FromStr for Translator {
    type Err = ParseTranslatorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut translator = HashMap::new();
        s.lines().enumerate().for_each(|(i, l)| {
            if i != 0 {
                let [dest, source, len] = match l.split(" ").collect::<Vec<_>>()[..] {
                    [dest, source, len] => [
                        dest.parse().unwrap_or(0),
                        source.parse().unwrap_or(0),
                        len.parse().unwrap_or(0),
                    ],
                    _ => [0, 0, 0],
                };
                let range: Range<u64> = source..source + len;
                let translation = dest as i64 - source as i64;
                translator.insert(range, translation);
            }
        });
        Ok(Translator(translator))
    }
}

impl Translator {
    fn translate(&self, from: u64) -> u64 {
        let table = &self.0;
        let mut to: u64 = from;
        for (k, v) in table.iter() {
            if k.contains(&from) {
                to = (from as i64 + v) as u64;
                break;
            }
        }
        to.clone()
    }
}

type SeedToSoil = Translator;
type SoilToFertilizer = Translator;
type FertilizerToWater = Translator;
type WaterToLight = Translator;
type LightToTemperature = Translator;
type TemperatureToHumidity = Translator;
type HumidityToLocation = Translator;

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, s2so, so2f, f2w, w2li, li2t, t2h, h2lo) =
        match input.split("\n\n").collect::<Vec<_>>()[..] {
            [a, b, c, d, e, f, g, h] => (
                Seeds::from_str(a).ok().unwrap(),
                SeedToSoil::from_str(b).ok().unwrap(),
                SoilToFertilizer::from_str(c).ok().unwrap(),
                FertilizerToWater::from_str(d).ok().unwrap(),
                WaterToLight::from_str(e).ok().unwrap(),
                LightToTemperature::from_str(f).ok().unwrap(),
                TemperatureToHumidity::from_str(g).ok().unwrap(),
                HumidityToLocation::from_str(h).ok().unwrap(),
            ),
            _ => (
                Seeds::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
            ),
        };
    let soln = seeds
        .0
        .iter()
        .map(|s| {
            let so = s2so.translate(*s);
            let f = so2f.translate(so);
            let w = f2w.translate(f);
            let li = w2li.translate(w);
            let t = li2t.translate(li);
            let h = t2h.translate(t);
            let lo = h2lo.translate(h);
            lo
        })
        .min()
        .unwrap_or(0);
    Some(soln)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, s2so, so2f, f2w, w2li, li2t, t2h, h2lo) =
        match input.split("\n\n").collect::<Vec<_>>()[..] {
            [a, b, c, d, e, f, g, h] => (
                SeedPairs::from_str(a).ok().unwrap(),
                SeedToSoil::from_str(b).ok().unwrap(),
                SoilToFertilizer::from_str(c).ok().unwrap(),
                FertilizerToWater::from_str(d).ok().unwrap(),
                WaterToLight::from_str(e).ok().unwrap(),
                LightToTemperature::from_str(f).ok().unwrap(),
                TemperatureToHumidity::from_str(g).ok().unwrap(),
                HumidityToLocation::from_str(h).ok().unwrap(),
            ),
            _ => (
                SeedPairs::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
                Translator::from_str("").ok().unwrap(),
            ),
        };
    let soln = seeds
        .0
        .iter()
        .map(|s| {
            let so = s2so.translate(*s);
            let f = so2f.translate(so);
            let w = f2w.translate(f);
            let li = w2li.translate(w);
            let t = li2t.translate(li);
            let h = t2h.translate(t);
            let lo = h2lo.translate(h);
            lo
        })
        .min()
        .unwrap_or(0);
    Some(soln)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
