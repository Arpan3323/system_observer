
use std::io::{Result};
mod app;
mod processes;

/*
 * enter the Alternate Screen when starting and leave it when exiting 
 * and also enable raw mode to disable line buffering and enable reading key events
 */

fn main() -> Result<()>
{
    app::App::new().run()?;
    Ok(())
	
}

