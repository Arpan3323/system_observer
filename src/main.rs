use std::io::Result;
mod app;
mod system_info;
mod ui;

fn main() -> Result<()>
{
    app::App::new().run()?;
    Ok(())
}

#[test]
fn debugger()
{
    use sysinfo::Components;

    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!("{} {}°C", component.label(), component.temperature(),);
    }
    //Clears the console screen
    //Copied from stack overflow ngl :P
    print!("\x1B[2J\x1B[1;1H");
}
