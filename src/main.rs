use std::io::Result;
mod app;
mod system_info;
mod ui;

fn main() -> Result<()>
{
    //let shared_proc_info: Arc<Processes> = Arc::new(Processes::new());
    //let my_arc_struct: Arc<MyStruct> = Arc::new(my_struct);
    app::App::new().run()?;
    Ok(())
}

#[test]
fn debugger()
{
    use sysinfo::System;

    let s = System::new_all();
    println!("{:?}", s.cpus().len());
    //Clears the console screen
    //Copied from stack overflow ngl :P
    //print!("\x1B[2J\x1B[1;1H");
}
