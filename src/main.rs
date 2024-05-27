
use std::io::{Result};
mod app;
mod system_info;
mod ui;
//mod processes;

use std::{collections::HashMap, time::Duration};

use sysinfo::{System, RefreshKind,CpuRefreshKind, NetworkData, Networks};


#[tokio::main]
async fn main() -> Result<()>
{
    app::App::new().await.run()?;
    Ok(())
	
}

#[test]
fn debugger()

            {
                /*
                
                let mut s = System::new_with_specifics(
                    RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
                );
                loop {
                    
                    // Wait a bit because CPU usage is based on diff.
                    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
                    // Refresh CPUs again.
                    s.refresh_cpu();
                    
                    for cpu in s.cpus() {
                        //println!("{}", cpu.cpu_usage());
                        //println!("{}", cpu.name());
                        //println!("{}", cpu.brand());
                        println!("{}", cpu.frequency());
                        
                        
                        
                        
                    }
                    print!("\x1B[2J\x1B[1;1H"); 
                    //println!("{}", s.global_cpu_info().frequency());
                    print!("\x1B[2J\x1B[1;1H");
                }
                 */
                
            /*
            let s = System::new_all();
            println!("Total RAM: {} MB", s.total_memory() / 1000000);
            println!("Used RAM: {} MB", s.used_memory() / 1000000);
            */
            
            use sysinfo::Networks;

            //use sysinfo::Networks;

            use sysinfo::Components;

            let components = Components::new_with_refreshed_list();
            for component in &components {
                println!("{} {}°C", component.label(), component.temperature(),);
            }
            //println!("{}", s.global_cpu_info().vendor_id());
            //println!("aa");
            //print!("\x1B[2J\x1B[1;1H");
            }
