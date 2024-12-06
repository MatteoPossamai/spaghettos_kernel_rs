use super::colors::{Color, ColorCode};

pub struct ConsoleWriter {
    terminal_start: *mut u8,
    cell: usize,
    screen_width: usize,
    screen_height: usize,
}

impl ConsoleWriter {
    pub fn default() -> ConsoleWriter {
        ConsoleWriter {
            terminal_start: 0xB8000 as *mut u8,
            screen_width: 80,
            screen_height: 25,
            cell: 0,
        }
    }

    pub fn new(
        terminal_start: Option<*mut u8>,
        screen_width: Option<usize>,
        screen_height: Option<usize>,
    ) -> ConsoleWriter {
        let terminal_start = terminal_start.unwrap_or(0xB8000 as *mut u8);
        let screen_width = screen_width.unwrap_or(80);
        let screen_height = screen_height.unwrap_or(25);

        ConsoleWriter {
            terminal_start,
            screen_width, // Fixed from screen_height
            screen_height,
            cell: 0,
        }
    }

    fn MAX_CELLS(&self) -> usize {
        self.screen_height * self.screen_width
    }

    pub unsafe fn clear_ui(&mut self) -> () {
        let mut i: i32 = 0;

        self.cell = 0;

        while i < (2 * self.MAX_CELLS() as i32) {
            *(self.terminal_start.offset(i as isize)) = b' ';
            i += 2;
        }
    }

    pub unsafe fn set_monitor_color(&mut self, color: Color) {
        let mut i: i32 = 1;
        let color_code = ColorCode::new(Color::Black, color);

        while i < (2 * 80 * 25) {
            *(self.terminal_start.offset(i as isize)) = color_code.0;
            i += 2;
        }
    }

    pub unsafe fn scroll(&mut self) {
        for i in 0..self.screen_height - 1 {
            for j in 0..self.screen_width {
                let src = self
                    .terminal_start
                    .offset(((i + 1) * self.screen_width + j) as isize);
                let dst = self
                    .terminal_start
                    .offset((i * self.screen_width + j) as isize);
                *dst = *src;
            }
        }

        // Clear the last row after scrolling
        for j in 0..self.screen_width {
            let last_row = self
                .terminal_start
                .offset(((self.screen_height - 1) * self.screen_width + j) as isize);
            *last_row = b' '; // Clear the last row by writing spaces
        }

        self.cell -= self.screen_width; // Move CELL to the start of the last row
    }

    pub unsafe fn print_char(&mut self, c: char) {
        // Check if we reached the bottom of the screen
        if self.cell == self.screen_height * self.screen_width {
            self.scroll();
        }

        // Handle new line
        if c == '\n' {
            // Move to the next line
            self.cell = (self.cell + self.screen_width) - (self.cell % self.screen_width);
            return;
        }

        // Print the character at the current cell
        *(self.terminal_start.offset(self.cell as isize)) = c as u8;

        // Move to the next character position (each character takes 2 bytes)
        self.cell += 2;
    }

    pub unsafe fn print_string(&mut self, string: &str) {
        // Print all the chars in the string
        for char in string.chars().into_iter() {
            self.print_char(char);
        }
    }

    pub unsafe fn print_color_char(&mut self, c: char, co: char) {
        // Check if we reached the bottom of the screen
        if self.cell == self.MAX_CELLS() {
            self.scroll();
        }

        // Handle new line
        if c == '\n' {
            // Move to the next line
            self.cell = (self.cell + self.screen_width) - (self.cell % self.screen_width);
            return;
        }

        // Print the character at the current cell
        *(self.terminal_start.offset(self.cell as isize)) = c as u8;

        // Print the color attribute at the next byte (next position)
        *(self.terminal_start.offset((self.cell + 1) as isize)) = co as u8;

        // Move to the next character position (each character takes 2 bytes: char + color)
        self.cell += 2;
    }

    pub unsafe fn print_color_string(&mut self, string: &str, color: char) {
        for char in string.chars().into_iter() {
            self.print_color_char(char, color);
        }
    }
}
