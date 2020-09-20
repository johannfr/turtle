
use std::collections::HashMap;

#[derive(Debug)]
pub enum Stmt {
    PenUp,
    PenDown,
    Forward(i32),
    Backward(i32),
    Turn(i32),
    ChangeColor(Color),
    ChangeColorRGB(i32, i32, i32),
    Label(String),
    Loop(String, i32),
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Black,
}

#[derive(Debug, Copy, Clone)]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
pub struct Program {
    statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Program { statements }
    }

    pub fn dump(&self) {
        for stmt in self.statements.iter() {
            println!("{:?}", stmt);
        }
    }

    pub fn interpret(&self) -> Result<Vec<Line>, String>{
        return Interpreter::new(self).execute();
    }
}

struct Interpreter<'program> {
    source: &'program Vec<Stmt>,
}

impl<'program> Interpreter<'program> {
    fn new(program: &'program Program) -> Self {
        Interpreter {
            source: &program.statements,
        }
    }

    fn execute(&mut self) -> Result<Vec<Line>, String> {
        let mut pc: usize = 0;
        let mut labels = HashMap::new();
        let mut loop_store: HashMap<usize, i32> = HashMap::new();
        let mut world: Vec<Line> = Vec::new();
        let mut turtle = Turtle::new(
            Point {
                x: 0,
                y: 0,
            },
            0,              // start_heading
            ColorRGB {      // start_color
                r: 0,
                g: 0,
                b: 0,
            },
            true            // start_pendown
        );
        while let Some(stmt) = self.source.get(pc) {
            match stmt {
                &Stmt::PenUp => turtle.pen_down = false,
                &Stmt::PenDown => turtle.pen_down = true,
                &Stmt::Forward(n) => match turtle.forward(n as isize) {
                    Some(line) => world.push(line),
                    None => (),
                },
                &Stmt::Turn(n) => turtle.heading += (n % 360) as isize,
                &Stmt::ChangeColor(c) => turtle.color = match c {
                    Color::Red => ColorRGB{r: 255, g: 0, b: 0},
                    Color::Green => ColorRGB{r: 0, g: 255, b: 0},
                    Color::Blue => ColorRGB{r: 0, g: 0, b: 255},
                    Color::Black => ColorRGB{r: 0, g: 0, b: 0},
                },
                &Stmt::ChangeColorRGB(r, g, b) => turtle.color = ColorRGB{
                    r: r as u8,
                    g: g as u8,
                    b: b as u8
                },
                Stmt::Label(s) => { labels.entry(s).or_insert(pc); },
                Stmt::Loop(l, c) => {
                        *loop_store.entry(pc).or_insert(*c + 1) -= 1;
                        let counter = loop_store.get(&pc).unwrap();
                        if *counter <= 0 {
                            loop_store.remove(&pc);
                        }
                        else
                        {
                            pc = *labels.get(l).unwrap();
                        }
                },
                other => return Err(format!("Unimplemented statement: {:?}", other)),
            }
            pc += 1;
        }
        Ok(world)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub color: ColorRGB,
}

struct Turtle {
    location: Point,
    heading: isize,
    color: ColorRGB,
    pen_down: bool,
}

impl Turtle {
    fn new(start_location: Point, start_heading: isize, start_color: ColorRGB, start_pendown: bool) -> Self {
        Turtle {
            location: start_location,
            heading: start_heading,
            color: start_color,
            pen_down: start_pendown,
        }
    }

    fn forward(&mut self, distance: isize) -> Option<Line> {
        let old_location = self.location;
        self.location = Point{
            x: old_location.x + ((self.heading as f64).to_radians().cos() * distance as f64) as isize,
            y: old_location.y + ((self.heading as f64).to_radians().sin() * distance as f64) as isize,
        };

        match self.pen_down {
            true => Some(Line{start: old_location, end: self.location, color: self.color}),
            false => None,
        }
    }
}
