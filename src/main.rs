extern crate image;
extern crate rand;

use std::{env};
use image::{GrayImage, ImageBuffer};
use rand::{rngs::ThreadRng, seq::SliceRandom};


pub struct Maze {
    size: (usize, usize),
    map: Vec<Vec<bool>>,
    roads: Vec<(usize, usize)>,
    rng: ThreadRng,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut maze = Maze {
            size: (width, height),
            map: vec![vec![false; width]; height],
            roads: vec![],
            rng: rand::thread_rng(),
        };
        maze.create();
        return maze;
    }
    fn create(&mut self) -> &mut Maze {
        let (width, height) = self.size;
        let odd_intermediate_value =
            |num| if num / 2 % 2 == 0 { num / 2 + 1 } else { num / 2 };
        let start_point = (odd_intermediate_value(width), odd_intermediate_value(height));
        self.map[start_point.1][start_point.0] = true;
        self.roads.push(start_point);
        while self.roads.len() > 0 {
            let (x, y) = self.roads.pop().unwrap();
            self.way((x as i64, y as i64));
        }
        return self;
    }
    fn way(&mut self, position: (i64, i64)) {
        let (width, height) = (self.size.0, self.size.1);
        let (mut x, mut y) = position;
        loop {
            let mut around: Vec<i64> = vec![];
            if x - 2 > 0 && !self.map[y as usize][x as usize - 2] {
                around.push(-2);
            }
            if x + 2 < width as i64 && !self.map[y as usize][x as usize + 2] {
                around.push(0);
            }
            if y - 2 > 0 && !self.map[y as usize - 2][x as usize] {
                around.push(-1);
            }
            if y + 2 < height as i64 && !self.map[y as usize + 2][x as usize] {
                around.push(1);
            }
            if around.len() == 0 {
                break;
            }
            let r = *around.choose(&mut self.rng).unwrap();
            let vector = ((r + 1) % 2, r % 2);
            self.map[(y + vector.1 * 2) as usize][(x + vector.0 * 2) as usize] = true;
            self.map[(y + vector.1) as usize][(x + vector.0) as usize] = true;
            x += vector.0 * 2;
            y += vector.1 * 2;
            self.roads.push((x as usize, y as usize));
        }
    }
    pub fn draw(&self, path: &str) {
        let (width, height) = (self.size.0, self.size.1);
        let mut img: GrayImage = ImageBuffer::new(width as u32, height as u32);
        let pixel_white = image::Luma([255]);
        let pixel_black = image::Luma([0]);
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x as u32, y as u32,
                              if self.map[y][x] { pixel_white } else { pixel_black })
            }
        }
        img.save(path).unwrap();
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let width = args[1].parse().expect("Must be number");
    let height = args[2].parse().expect("Must be number");
    let path = args[3].as_str();
    let odd = |num| if num % 2 == 0 { num - 1 } else { num };
    Maze::new(odd(width), odd(height)).draw(path);
}

