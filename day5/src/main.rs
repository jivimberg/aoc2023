use std::collections::HashSet;
use std::ops::Range;

use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{digit1, line_ending, newline, space1};
use nom::combinator::{map, map_res, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{pair, tuple};
use itertools::Itertools;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    destination: u64,
    source_range: Range<u64>
}

fn parse_almanac(s: &str) -> IResult<&str, Almanac> {
    let f = tuple((
            parse_seeds,
            newline,
            parse_map,
            parse_map,
            parse_map,
            parse_map,
            parse_map,
            parse_map,
            parse_map,
        ));
    map(f, |(seeds, _, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location)| {
            Almanac { seeds, seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location }
        })(s)
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<Range<u64>>> {
    let f = tuple((
        tag("seeds: "),
        separated_list1(space1, map_res(digit1, str::parse)),
        newline
    ));
    map(f, |(_, numbers, _)| {
        numbers.into_iter()
            .tuples()
            .map(|(start, end)| start..start + end)
            .collect()
    })(s)
}

fn parse_map(s: &str) -> IResult<&str, Vec<Mapping>> {
    let f = tuple((
            pair(take_till(|c| c == '\n'), newline),
            separated_list1(line_ending, parse_mapping), opt(newline),
            opt(newline)
        ));
    map(f, |(_, mapping, _, _)| mapping)(s)
}

fn parse_mapping(s: &str) -> IResult<&str, Mapping> {
    let f = separated_list1(space1, map_res(digit1, str::parse));
    map(f, |m| {
        let destination = m[0];
        let source_range = m[1]..m[1] + m[2];
        Mapping { destination, source_range }
    })(s)
}

fn convert(number: u64, mappings: &Vec<Mapping>) -> u64 {
    mappings.iter()
        .find(|mappings| mappings.source_range.contains(&number))
        .map_or(number, |mapping| mapping.destination + (number - mapping.source_range.start))
}

fn main() {
    let input: &str = include_str!("sample_input.txt");
    let almanac = parse_almanac(input).unwrap().1;
    let mut seeds: HashSet<u64> = HashSet::new();
    dbg!("finished parsing");
    for seed_range in almanac.seeds.iter() {
        seeds.extend(seed_range.clone().into_iter());
    }
    dbg!(&seeds.len());
    let result: u64 = seeds.iter()
        .map(|seed| {
            let soil = convert(*seed, &almanac.seed_to_soil);
            let fertilizer = convert(soil, &almanac.soil_to_fertilizer);
            let water = convert(fertilizer, &almanac.fertilizer_to_water);
            let light = convert(water, &almanac.water_to_light);
            let temperature = convert(light, &almanac.light_to_temperature);
            let humidity = convert(temperature, &almanac.temperature_to_humidity);
            let location = convert(humidity, &almanac.humidity_to_location);
            location
        })
        .min()
        .unwrap();

    dbg!(result);
}
