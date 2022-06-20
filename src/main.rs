mod app;
mod backend;
mod ui;

use std::{io, time::Duration};

use crate::backend::crossterm::run;

fn main() -> Result<(), io::Error> {
    let tick_rate = Duration::from_millis(250);

    run(tick_rate)?;

    Ok(())
}
