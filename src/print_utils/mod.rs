use crate::assetts;

static mut TERMINAL_START: *mut u8 = 0xB8000 as *mut u8; // Pointer to the start of the video memory
static mut CELL: usize = 0; // Current cell position

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 25;
const MAX_CELLS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
// static LAST_CELL: usize = MAX_CELLS * 2; // Last cell in

pub unsafe fn print_start() -> i32 {
    // Restart the pointer to the top of the Window
    CELL = 0;

    // Make the UI basic printing
    clear_ui();
    set_monitor_color('a');
    print_string(assetts::CREDENTIALS);
    print_string("");

    print_color_string(assetts::DEFAULT_CLI, '1');

    0
}

pub fn clear_ui() {
    let mut i: i32 = 0;

    unsafe {
        CELL = 0;
    }

    while i < (2 * 80 * 25) {
        unsafe {
            *(TERMINAL_START.offset(i as isize)) = b' ';
            i += 2;
        }
    }
}

fn set_monitor_color(color: char) {
    let mut i: i32 = 1;

    while i < (2 * 80 * 25) {
        unsafe {
            *(TERMINAL_START.offset(i as isize)) = color as u8;
            i += 2;
        }
    }
}

// Function to scroll the screen
unsafe fn scroll() {
    for i in 0..SCREEN_HEIGHT - 1 {
        for j in 0..SCREEN_WIDTH {
            let src = TERMINAL_START.offset(((i + 1) * SCREEN_WIDTH + j) as isize);
            let dst = TERMINAL_START.offset((i * SCREEN_WIDTH + j) as isize);
            *dst = *src;
        }
    }

    // Clear the last row after scrolling
    for j in 0..SCREEN_WIDTH {
        let last_row = TERMINAL_START.offset(((SCREEN_HEIGHT - 1) * SCREEN_WIDTH + j) as isize);
        *last_row = b' '; // Clear the last row by writing spaces
    }

    CELL -= SCREEN_WIDTH; // Move CELL to the start of the last row
}

// Function to print a character
unsafe fn print_char(c: char) {
    // Check if we reached the bottom of the screen
    if CELL == MAX_CELLS {
        scroll();
    }

    // Handle new line
    if c == '\n' {
        // Move to the next line
        CELL = (CELL + SCREEN_WIDTH) - (CELL % SCREEN_WIDTH);
        return;
    }

    // Print the character at the current cell
    *(TERMINAL_START.offset(CELL as isize)) = c as u8;

    // Move to the next character position (each character takes 2 bytes)
    CELL += 2;
}

unsafe fn print_string(string: &str) {
    // Print all the chars in the string
    for char in string.chars().into_iter() {
        print_char(char);
    }
}

// Function to print a character with color
unsafe fn print_color_char(c: char, co: char) {
    // Check if we reached the bottom of the screen
    if CELL == MAX_CELLS {
        scroll();
    }

    // Handle new line
    if c == '\n' {
        // Move to the next line
        CELL = (CELL + SCREEN_WIDTH) - (CELL % SCREEN_WIDTH);
        return;
    }

    // Print the character at the current cell
    *(TERMINAL_START.offset(CELL as isize)) = c as u8;

    // Print the color attribute at the next byte (next position)
    *(TERMINAL_START.offset((CELL + 1) as isize)) = co as u8;

    // Move to the next character position (each character takes 2 bytes: char + color)
    CELL += 2;
}

unsafe fn print_color_string(string: &str, color: char) {
    for char in string.chars().into_iter() {
        print_color_char(char, color);
    }
}
