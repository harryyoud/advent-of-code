use aoc_2023::get_input;
use aoc_lib::paragraphs::Paragraphs;
use itertools::Itertools;
use rangemap::{RangeMap, RangeSet};
use std::marker::PhantomData;

// This solution is quite verbose mostly because we use individual types for each map
// We could cut down number of lines quite a lot by using overlapping types, but that's boring

fn main() {
    let input = get_input(5);

    let mut lines = input.paragraphs();

    let seeds = lines
        .next()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let resolver = FullResolver {
        seed_soil_map: parse_map(lines.next().unwrap()),
        soil_fertilizer_map: parse_map(lines.next().unwrap()),
        fertilizer_water_map: parse_map(lines.next().unwrap()),
        water_light_map: parse_map(lines.next().unwrap()),
        light_temperature_map: parse_map(lines.next().unwrap()),
        temperature_humidity_map: parse_map(lines.next().unwrap()),
        humidity_location_map: parse_map(lines.next().unwrap()),
    };

    let part_1 = seeds
        .iter()
        .map(|seed| resolver.full_resolve(&Seed::from_u64(*seed)))
        .min()
        .unwrap()
        .to_u64();

    dbg!(part_1);

    let seeds = seeds
        .into_iter()
        .tuples()
        .map(|(l, r)| l..(l + r))
        .collect::<RangeSet<_>>();

    let part_2 = (0_u64..)
        .filter(|n| seeds.contains(&resolver.reverse_resolve(&Location(*n)).0))
        .next()
        .unwrap();

    dbg!(part_2);
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
                            &self
                                .soil_fertilizer_map
                                .resolve(&self.seed_soil_map.resolve(seed)),
                        ),
                    ),
                ),
            ),
        )
    }

    fn reverse_resolve(&self, location: &Location) -> Seed {
        self.seed_soil_map.reverse_resolve(
            &self.soil_fertilizer_map.reverse_resolve(
                &self.fertilizer_water_map.reverse_resolve(
                    &self.water_light_map.reverse_resolve(
                        &self.light_temperature_map.reverse_resolve(
                            &self.temperature_humidity_map.reverse_resolve(
                                &self.humidity_location_map.reverse_resolve(location),
                            ),
                        ),
                    ),
                ),
            ),
        )
    }
}

fn parse_map<'a, K, V>(mut lines: impl Iterator<Item = &'a str>) -> RangeMapResolver<K, V>
where
    K: RangeKey,
    V: RangeKey,
{
    assert_eq!(
        lines.next().unwrap(),
        format!(
            "{}-to-{} map:",
            std::any::type_name::<K>()
                .to_lowercase()
                .split("::")
                .last()
                .unwrap(),
            std::any::type_name::<V>()
                .to_lowercase()
                .split("::")
                .last()
                .unwrap(),
        ),
        "Unexpected map in input"
    );
    let forward = lines
        .take_while(|line| !line.is_empty())
        .map(|s| {
            s.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(destination_start, source_start, range_length)| {
            (
                source_start..(source_start + range_length),
                RangeMapInformation {
                    destination_start,
                    source_start,
                    range_length,
                },
            )
        })
        .collect();
    RangeMapResolver::from_range_map(forward)
}

struct RangeMapResolver<K, V>
where
    K: RangeKey,
    V: RangeKey,
{
    _marker1: PhantomData<K>,
    _marker2: PhantomData<V>,
    forward: RangeMap<u64, RangeMapInformation>,
    reverse: RangeMap<u64, RangeMapInformation>,
}

impl<K, V> RangeMapResolver<K, V>
where
    K: RangeKey,
    V: RangeKey,
{
    fn from_range_map(forward: RangeMap<u64, RangeMapInformation>) -> Self {
        Self {
            _marker1: PhantomData,
            _marker2: PhantomData,
            reverse: reverse_map(forward.clone()),
            forward,
        }
    }

    fn resolve(&self, key: &K) -> V {
        let rmi = self.forward.get(&key.to_u64());
        let Some(rmi) = rmi else {
            return V::from_u64(key.to_u64());
        };
        V::from_u64(key.to_u64() - rmi.source_start + rmi.destination_start)
    }

    fn reverse_resolve(&self, key: &V) -> K {
        let rmi = self.reverse.get(&key.to_u64());
        let Some(rmi) = rmi else {
            return K::from_u64(key.to_u64());
        };
        K::from_u64(key.to_u64() - rmi.source_start + rmi.destination_start)
    }
}

fn reverse_map(forward: RangeMap<u64, RangeMapInformation>) -> RangeMap<u64, RangeMapInformation> {
    forward
        .into_iter()
        .map(|(_, rmi)| {
            (
                rmi.destination_start..(rmi.destination_start + rmi.range_length),
                RangeMapInformation {
                    source_start: rmi.destination_start,
                    destination_start: rmi.source_start,
                    range_length: rmi.range_length,
                },
            )
        })
        .collect()
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
