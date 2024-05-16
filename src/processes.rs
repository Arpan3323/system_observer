use sysinfo::{System, Pid, Components, Disks, Networks};
use std::collections::BinaryHeap;
//use sysinfo::Pid;

//static sys: sysinfo::System = System::new_all();

#[derive(Debug)]
pub struct Process
{
    pub name: String,
    pub pid: u32,
    pub status: String,
    pub memory_usage: u64,
    pub cpu_usage: f32,
}

pub struct Processes 
{
   pub all_procs: Vec<Process>
}


impl Processes {

    pub fn new() -> Processes
    {
        Processes
        {
            all_procs: Processes::get_all_procs()       
        }
    }

    fn get_all_procs() -> Vec<Process> 
    {
        let mut sys = System::new_all();
        let mut all_procs: Vec<Process> = Vec::new();
        sys.refresh_all();
        sys.refresh_all();
        for (pid, process)in sys.processes() 
        {
            let curr_proc = Process {
                name: process.name().to_string(),
                pid: pid.as_u32(),
                status: process.status().to_string(),
                memory_usage: process.memory() / 1000000,
                cpu_usage: process.cpu_usage(),
            };
            all_procs.push(curr_proc);
        }
        
        //all_procs.sort_by(|a,b|b.cmp(a));
        all_procs
    }
}


    
