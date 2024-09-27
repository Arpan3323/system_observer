use std::io::Result;
mod app;
mod system_info;
mod ui;

fn main() -> Result<()>
{
    app::App::new().run()?;
    Ok(())
}