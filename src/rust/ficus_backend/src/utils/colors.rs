use std::collections::HashSet;

use rand::Rng;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn black() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn random(used: Option<&HashSet<Color>>) -> Color {
        let mut rng = rand::thread_rng();

        loop {
            let red = rng.gen::<u8>();
            let green = rng.gen::<u8>();
            let blue = rng.gen::<u8>();

            let color = Self::new(red, green, blue);
            if let Some(used_colors) = used {
                if !used_colors.contains(&color) {
                    return color;
                }
            } else {
                return color;
            }
        }
    }

    pub fn red(&self) -> u8 {
        self.red
    }
    pub fn green(&self) -> u8 {
        self.green
    }
    pub fn blue(&self) -> u8 {
        self.blue
    }
}
