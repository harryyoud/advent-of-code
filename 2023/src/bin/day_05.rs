use std::{marker::PhantomData, str::Lines};

use aoc_2023::get_input;
use indicatif::{ParallelProgressIterator, ProgressBar};
use itertools::Itertools;
use rangemap::RangeMap;
use rayon::prelude::*;

struct RangeMapResolver<K, V> where K: RangeKey, V: RangeKey {
    _marker1: PhantomData<K>,
    _marker2: PhantomData<V>,
    inner: RangeMap<u64, RangeMapInformation>,
}

impl<K, V> RangeMapResolver<K, V> where K: RangeKey, V: RangeKey {
    fn from_range_map(map: RangeMap<u64, RangeMapInformation>) -> Self {
        Self {
            _marker1: PhantomData,
            _marker2: PhantomData,
            inner: map,
        }
    }

    fn resolve(&self, key: &K) -> V {
        let rmi = self.inner.get(&key.to_u64());
        let Some(rmi) = rmi else {
            return V::from_u64(key.to_u64());
        };
        V::from_u64(key.to_u64() - rmi.source_start + rmi.destination_start)
    }
}

struct FullResolver {
    seed_soil_map: RangeMapResolver<Seed, Soil>,
    soil_fertilizer_map: RangeMapResolver<Soil, Fertilizer>,
    fertilizer_water_map: RangeMapResolver<Fertilizer, Water>,
    water_light_map: RangeMapResolver<Water, Light>,
    light_temperature_map: RangeMapResolver<Light, Temperature>,
    temperature_humidity_map: RangeMapResolver<Temperature, Humidity>,
    humidity_location_map: RangeMapResolver<Humidity, Location>,
}

impl FullResolver {
    fn full_resolve(&self, seed: &Seed) -> Location {
        self.humidity_location_map.resolve(
            &self.temperature_humidity_map.resolve(
                &self.light_temperature_map.resolve(
                    &self.water_light_map.resolve(
                        &self.fertilizer_water_map.resolve(
                            &self.soil_fertilizer_map.resolve(
                                &self.seed_soil_map.resolve(seed)
                            )
                        )
                    )
                )
            )
        )
    }
}
 

fn main() {
    let input = get_input(5);
    let mut lines = input.lines();

    let seeds_line = lines.next().unwrap();
    let _ = lines.next().unwrap(); // remove blank line ready for next section of input

    let resolver = FullResolver {
        seed_soil_map: parse_map(&mut lines),
        soil_fertilizer_map: parse_map(&mut lines),
        fertilizer_water_map: parse_map(&mut lines),
        water_light_map: parse_map(&mut lines),
        light_temperature_map: parse_map(&mut lines),
        temperature_humidity_map: parse_map(&mut lines),
        humidity_location_map: parse_map(&mut lines),
    };

    dbg!(seeds_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| Seed::from_u64(s.parse::<u64>().unwrap()))
        .map(|seed| resolver.full_resolve(&seed))
        .min()
    );

    let size: u64 = seeds_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .skip(1)
        .step_by(2)
        .sum();

    let progress = ProgressBar::new(size);

    dbg!(
        seeds_line
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .tuples()
        .filter_map(|(start, length)| {
            (start..(start + length))
                .into_par_iter()
                .progress_with(progress.clone())
                .map(Seed::from_u64)
                .map(|seed| resolver.full_resolve(&seed))
                .min()
        })
        .min()
    );
}

fn parse_map<'a, K, V>(lines: &mut Lines) -> RangeMapResolver<K, V> where K: RangeKey, V: RangeKey {
    let mut map: RangeMap<u64, RangeMapInformation> = RangeMap::new();
    assert_eq!(
        lines.next().unwrap(),
        format!("{}-to-{} map:",
            std::any::type_name::<K>().to_lowercase().split("::").last().unwrap(),
            std::any::type_name::<V>().to_lowercase().split("::").last().unwrap(),
        )
    );
    for rmi in lines
        .take_while(|line| !line.is_empty())
        .map(|s|
            s.split_whitespace().map(
                |x| x.parse::<u64>().unwrap()
            ).collect_tuple().unwrap()
        )
        .map(|(destination_start, source_start, range_length)|
            RangeMapInformation { destination_start, source_start, range_length }
        )
    {
        map.insert(rmi.source_start..(rmi.source_start + rmi.range_length), rmi)
    }
    RangeMapResolver::from_range_map(map)
}

pub trait RangeKey {
    fn from_u64(from: u64) -> Self;
    fn to_u64(&self) -> u64;
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Seed(u64);
impl RangeKey for Seed {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Soil(u64);
impl RangeKey for Soil {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Fertilizer(u64);
impl RangeKey for Fertilizer {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Water(u64);
impl RangeKey for Water {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Light(u64);
impl RangeKey for Light {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Temperature(u64);
impl RangeKey for Temperature {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Humidity(u64);
impl RangeKey for Humidity {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct Location(u64);
impl RangeKey for Location {
    fn from_u64(from: u64) -> Self {
        Self(from)
    }
    fn to_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone)]
pub struct RangeMapInformation {
    pub source_start: u64,
    pub destination_start: u64,
    pub range_length: u64,
}

