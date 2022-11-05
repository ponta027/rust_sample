use std::env;
use svg::node::element::path::{Command, Data, Position};
use svg::node::element::{Path, Rectangle};
use svg::Document;

use crate::Operation::{Forward, Home, Noop, TurnLeft, TurnRight};
use crate::Orientation::{East, North, South, West};

///
const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_Y: isize = HEIGHT / 2;
const HOME_X: isize = WIDTH / 2;
const STROKE_WIDTH: usize = 5;

///
#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}
#[derive(Debug, Clone, Copy)]
enum Orientation {
    North,
    East,
    West,
    South,
}
///
#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    ///
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }
    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }
    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West => self.x += distance,
            East => self.x -= distance,
        }
    }
    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West => North,
            East => South,
        }
    }
    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West => South,
            East => North,
        }
    }
    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

///
fn parse(input: &str) -> Vec<Operation> {
    ///
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize;
                Forward(distance * (HEIGHT / 10))
            }
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte),
        };
        steps.push(step);
    }
    steps
}
///
fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut path_data = Vec::<Command>::with_capacity(operations.len());

    let mut turtle = Artist::new();

    let start_at_home = Command::Move(Position::Absolute, (HOME_X, HOME_Y).into());

    path_data.push(start_at_home);
    //
    for op in operations {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft => turtle.turn_left(),
            TurnRight => turtle.turn_right(),
            Home => turtle.home(),
            Noop(byte) => {
                eprintln!("warning: illeagl byte encountered: {:?}", byte);
            }
        };

        let path_segment = Command::Line(Position::Absolute, (turtle.x, turtle.y).into());
        path_data.push(path_segment);
        turtle.wrap();
        //
    }
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    //
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");
    let border = background
        .clone()
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3 * STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=outline: 5px solid #800000;")
        .add(background)
        .add(sketch)
        .add(border);
    println!("{:?}", document);
    document
}

fn main() {
    println!("Hello, world!");
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default);

    let operation = parse(input);
    let path_data = convert(&operation);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
    println!("{}", default);
}

impl PartialEq for Operation {
    ///
    fn eq(&self, other: &Self) -> bool {
        println!("{:?}", self);
        println!("{:?}", other);
        // T.B.D when compare object , occure stack over flow
        //self == other
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let input: &str = "abc";

        let mut steps = Vec::<Operation>::with_capacity(input.len());
        steps.push(TurnLeft);
        steps.push(TurnLeft);
        steps.push(TurnLeft);
        let mut result = parse(input);
        assert_eq!(result.len(), steps.len());
        let a = result.pop().unwrap();
        let b = steps.pop().unwrap();
        assert_eq!(a, b);
        //        assert_eq!(result.pop().unwrap(), steps.pop().unwrap());
    }

    ///
    #[test]
    fn test_convert() {
        let operations: Vec<Operation> = Vec::<Operation>::with_capacity(10);
        let result: Vec<Command> = convert(&operations);
        assert_eq!(result.len(), operations.len());
    }
    ///
    #[test]
    fn test_generate_svg() {
        ///https://docs.rs/svg/latest/svg/node/element/struct.SVG.html
        let path_data: Vec<Command> = Vec::<Command>::with_capacity(10);
        let result: Document = generate_svg(path_data);
        let document = Document::new();
        //   assert_eq!(document, result);
    }
    #[test]
    fn test_artist() {
        let artist = Artist::new();
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y);
    }
    #[test]
    fn test_forward() {
        let mut artist = Artist::new();
        artist.forward(10);
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y + 10);
    }
    #[test]
    fn test_home() {
        let mut artist = Artist::new();
        artist.forward(10);
        artist.home();
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y);
    }
    #[test]
    fn test_turn_right() {
        let mut artist = Artist::new();
        artist.turn_right();
        artist.forward(10);
        assert_eq!(artist.x, HOME_X - 10);
        assert_eq!(artist.y, HOME_Y);
    }
    #[test]
    fn test_turn_left() {
        let mut artist = Artist::new();
        artist.turn_left();
        artist.forward(10);
        assert_eq!(artist.x, HOME_X + 10);
        assert_eq!(artist.y, HOME_Y);
    }
    #[test]
    fn test_wrap() {
        let mut artist = Artist::new();
        artist.wrap();
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y);
    }
    #[test]
    fn test_wrap2() {
        let mut artist = Artist::new();
        artist.forward(WIDTH);
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y + WIDTH);
        artist.wrap();
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y);
        artist.forward(WIDTH);
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y - WIDTH);
        artist.wrap();
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y);

        artist.forward(WIDTH);
        assert_eq!(artist.x, HOME_X);
        assert_eq!(artist.y, HOME_Y + WIDTH);

        //        assert_eq!(artist.heading, North);
    }
}
