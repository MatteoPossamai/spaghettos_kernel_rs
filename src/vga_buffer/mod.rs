use crate::assetts;
pub mod colors;
pub mod console_writer;

pub unsafe fn print_start() -> i32 {
    let mut writer = console_writer::ConsoleWriter::new(None, None, None);

    writer.clear_ui();
    writer.set_monitor_color(colors::Color::Magenta);
    writer.print_string(assetts::CREDENTIALS);

    0
}
