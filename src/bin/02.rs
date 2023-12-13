use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};

use aoc2023::read_input;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct ColoredBall {
    count: u32,
    color: Color,
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            _ => None,
        }
    }
}

#[derive(Default, Debug)]
struct Set {
    reds: u32,
    greens: u32,
    blues: u32,
}

impl Set {
    fn from(colors: Vec<ColoredBall>) -> Self {
        let mut s = Set::default();
        for ball in colors {
            match ball.color {
                Color::Red => s.reds = ball.count,
                Color::Green => s.greens = ball.count,
                Color::Blue => s.blues = ball.count,
            }
        }
        s
    }
}

struct Config {
    red_max: u32,
    green_max: u32,
    blue_max: u32,
}

fn main() {
    let input = read_input("./inputs/02.txt");

    let games = parse_input(&input);

    let config = Config {
        red_max: 12,
        green_max: 13,
        blue_max: 14,
    };
    let part_1 = games.iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                set.reds <= config.red_max && set.greens <= config.green_max && set.blues <= config.blue_max
            })
        })
        .map(|game| game.id)
        .sum::<u32>();

    let part_2 = games.iter()
        .map(|g| min_ball_power(g))
        .sum::<u32>();

    println!("part1: {}", part_1);
    println!("part2: {}", part_2);
    // println!("color_name: {:?}", parse_color("5 blue").unwrap().1);
}

fn min_ball_power(game: &Game) -> u32 {
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for set in &game.sets {
        if set.reds > max_red {
            max_red = set.reds;
        }
        if set.greens > max_green {
            max_green = set.greens;
        }
        if set.blues > max_blue {
            max_blue = set.blues;
        }
    }
    max_red * max_green * max_blue
}

fn parse_input(inp: &str) -> Vec<Game> {
    let res = separated_list0(newline, parse_game)(inp);

    res.unwrap().1
}

fn parse_game(inp: &str) -> nom::IResult<&str, Game> {
    map(
        separated_pair(
            preceded(tag("Game "), u32),
            tag(": "),
            separated_list0(tag("; "), parse_set),
        ),
        |(id, sets)| Game { id, sets },
    )(inp)
}

fn parse_set(inp: &str) -> nom::IResult<&str, Set> {
    map(
        separated_list0(tag(", "), parse_color),
        |colors| Set::from(colors),
    )(inp)
}

fn parse_color(inp: &str) -> nom::IResult<&str, ColoredBall> {
    map(
        separated_pair(u32, tag(" "), parse_color_name),
        |(count, color)| ColoredBall { count, color },
    )(inp)
}

fn parse_color_name(inp: &str) -> nom::IResult<&str, Color> {
    map(alt((tag("red"), tag("blue"), tag("green"))),
        |color_name| Color::from_str(color_name).unwrap(),
    )(inp)
}
