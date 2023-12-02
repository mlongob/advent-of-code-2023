#[derive(Clone, Debug, PartialEq)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl Game {
    fn playable(&self, reds: u32, greens: u32, blues: u32) -> bool {
        self.max_red <= reds && self.max_green <= greens && self.max_blue <= blues
    }

    fn cubes_power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let game = l.parse::<Game>().ok()?;
                match game.playable(12, 13, 14) {
                    true => Some(game.id),
                    false => None,
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|l| {
                let game = l.parse::<Game>().ok()?;
                Some(game.cubes_power())
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

mod input_parser {
    use super::Game;
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, space0, space1},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::tuple,
        Finish, IResult,
    };
    use std::str::FromStr;

    #[derive(Clone, Debug, PartialEq)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    fn number_u32(input: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse::<u32>)(input)
    }

    fn color(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag("red"), |_| Color::Red),
            map(tag("green"), |_| Color::Green),
            map(tag("blue"), |_| Color::Blue),
        ))(input)
    }

    fn hand(input: &str) -> IResult<&str, (Color, u32)> {
        map(
            tuple((space0, number_u32, space1, color)),
            |(_, n, _, c)| (c, n),
        )(input)
    }

    fn game(input: &str) -> IResult<&str, Game> {
        map(
            tuple((
                tag("Game"),
                space1,
                number_u32,
                tag(":"),
                separated_list0(alt((tag(","), tag(";"))), hand),
            )),
            |(_, _, id, _, l)| {
                let mut g = Game {
                    id,
                    max_red: 0,
                    max_green: 0,
                    max_blue: 0,
                };
                for (c, cnt) in l {
                    match c {
                        Color::Red => {
                            g.max_red = std::cmp::max(g.max_red, cnt);
                        }
                        Color::Blue => {
                            g.max_blue = std::cmp::max(g.max_blue, cnt);
                        }
                        Color::Green => {
                            g.max_green = std::cmp::max(g.max_green, cnt);
                        }
                    }
                }
                g
            },
        )(input)
    }

    impl FromStr for Game {
        type Err = nom::error::Error<String>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match game(s).finish() {
                Ok((_remaining, plan)) => Ok(plan),
                Err(nom::error::Error { input, code }) => Err(Self::Err {
                    input: input.to_string(),
                    code,
                }),
            }
        }
    }
}
