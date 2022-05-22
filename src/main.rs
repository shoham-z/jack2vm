mod xmlwriter;
mod tokenizer;
mod compilation_engine;

// include the latest version of the regex crate in your Cargo.toml
extern crate regex;
extern crate lazy_static;
use std::{env, fs};
use crate::tokenizer::tokenizer;
use compilation_engine::CompilationEngine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = search_jack_files(args[1].as_str());
    for file in file_path {
        tokenizer(file.to_string());
        let mut compilation_engine: CompilationEngine= CompilationEngine::new(&file);
        compilation_engine.compile_class();
        break;
    }
}

fn main1(){
    let content = "// This file is part of www.nand2tetris.org
// and the book \"The Elements of Computing Systems\"
// by Nisan and Schocken, MIT Press.
// File name: projects/10/Square/Square.jack

// (same as projects/09/Square/Square.jack)

/** Implements a graphical square. */
class Square {

   field int x, y; // screen location of the square's top-left corner
   field int size; // length of this square, in pixels

   /** Constructs a new square with a given location and size. */
   constructor Square new(int Ax, int Ay, int Asize) {
      let x = Ax;
      let y = Ay;
      let size = Asize;
      do draw();
      return this;
   }

   /** Disposes this square. */
   method void dispose() {
      do Memory.deAlloc(this);
      return;
   }

   /** Draws the square on the screen. */
   method void draw() {
      do Screen.setColor(true);
      do Screen.drawRectangle(x, y, x + size, y + size);
      return;
   }

   /** Erases the square from the screen. */
   method void erase() {
      do Screen.setColor(false);
      do Screen.drawRectangle(x, y, x + size, y + size);
      return;
   }

    /** Increments the square size by 2 pixels. */
   method void incSize() {
      if (((y + size) < 254) & ((x + size) < 510)) {
         do erase();
         let size = size + 2;
         do draw();
      }
      return;
   }

   /** Decrements the square size by 2 pixels. */
   method void decSize() {
      if (size > 2) {
         do erase();
         let size = size - 2;
         do draw();
      }
      return;
   }

   /** Moves the square up by 2 pixels. */
   method void moveUp() {
      if (y > 1) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
         let y = y - 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, y, x + size, y + 1);
      }
      return;
   }

   /** Moves the square down by 2 pixels. */
   method void moveDown() {
      if ((y + size) < 254) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, y, x + size, y + 1);
         let y = y + 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, (y + size) - 1, x + size, y + size);
      }
      return;
   }

   /** Moves the square left by 2 pixels. */
   method void moveLeft() {
      if (x > 1) {
         do Screen.setColor(false);
         do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
         let x = x - 2;
         do Screen.setColor(true);
         do Screen.drawRectangle(x, y, x + 1, y + size);
      }
      return;
   }

   /** Moves the square right by 2 pixels. */
   method void moveRight() {
      if ((x + size) < 510) {
         do Screen.setColor(false);
         do Screen.drawRectangle(x, y, x + 1, y + size);
         let x = x + 2;
         do Screen.setColor(true);
         do Screen.drawRectangle((x + size) - 1, y, x + size, y + size);
      }
      return;
   }
}";

    let lines = content.split("\n");

    for line in lines {
        //println!("{}", line.trim_start());

        if line.trim_start().starts_with("field") | line.trim_start().starts_with("static") {
            println!("var dec");
        }
    }

    let lines = content.split("\n");

    let mut func_body = false;
    let mut func_contents: String = "".to_string();
    for line in lines {
        let clear_line = line.trim_start();
        if clear_line.starts_with("function") | clear_line.starts_with("method") | clear_line.starts_with("constructor") {
            println!("subroutine dec: {}", line);
            func_body = true;
        } else if func_body {
            if line.contains("}") {
                for ch in line.chars() {
                    func_contents.push(ch.clone());
                }
                println!("subroutine: {}", line);
                func_body = false;
            }
            for ch in line.chars() {
                func_contents.push(ch);
            }
        }
    }

}


fn search_jack_files(file_path: &str) -> Vec<String> {
    let mut jack_files: Vec<String> = Vec::new();
    let paths = fs::read_dir(file_path).unwrap();
    for path in paths {
        if path.as_ref().unwrap().path().display().to_string().contains(".jack") {
            jack_files.push(path.unwrap().path().display().to_string())
        }
    }
    jack_files
}