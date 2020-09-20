extern crate turtle;

use std::fs::File;
use std::io::{Write, Error};

use turtle::ast::*;

//// draw_the_world(&vec!<Line>, ) - 
fn draw_the_world(world: &Vec<Line>) -> Result<(), Error> {
    let path = "svg_rendering.html";

    let mut output = File::create(path)?;
    let html_header = "<html><body><h1>Some SVG</h1>";
    let html_footer = "</body></html>";
    let svg_footer = "</svg>";

    write!(output, "{}\n", html_header)?;
    write!(output, "<svg width=\"{}\" height=\"{}\">\n", 200, 200)?;
    // write line from vector 
        for lines in world {
            write!(
                output,
                "<line
                    x1=\"{}\"
                    y1=\"{}\"
                    x2=\"{}\"
                    y2=\"{}\"
                    style=\"stroke:rgb({},{},{});stroke-width:2\"
                />\n",
                lines.start.x,
                lines.start.y,
                lines.end.x,
                lines.end.y,
                lines.color.r,
                lines.color.g,
                lines.color.b,
            )?;
        }     
    write!(output, "{}\n", svg_footer)?; 
    write!(output, "{}\n", html_footer)?;

    Ok(())
}

fn main() {
    use std::io::*;
    let mut source = String::new();
    match std::env::args().nth(1) {
        Some(filename) => {
            File::open(&filename)
                .expect(&format!("Can't open {}", &filename))
                .read_to_string(&mut source)
                .expect(&format!("Can't read {}", &filename));
        }

        None => {
            stdin()
                .read_to_string(&mut source)
                .expect("Can't read stdin");
        }
    }

    if source.is_empty() {
        println!("No input");
        return;
    }

    let world: Vec<turtle::ast::Line> =
    match turtle::compile(&source) {
        Ok(c) => {
            c.interpret().unwrap()
        },
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            std::process::exit(1);
        },
    };


    let status = draw_the_world(&world);

    match status {
        Ok(v) => println!("Done {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
}

