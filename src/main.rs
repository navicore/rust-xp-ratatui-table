mod tui;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tui::run()
}
