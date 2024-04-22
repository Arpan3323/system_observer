use sysinfo::{System, Pid, Components, Disks, Networks};
//use sysinfo::Pid;

//static sys: sysinfo::System = System::new_all();

pub struct Process{
name: String,
pid: u32,
memory_usage: u64,
parent_pid: u32,
status: String,
}

pub fn proc_mem_use(proc_pid: Pid, s: &System) -> u64{
	let mut res: u64 = 0;
	if let Some(process) = s.process(Pid::from(proc_pid)){
		res = process.memory();
	}
	println!("{:?}", res);
	return res;
} 

pub fn get_all_procs() -> Vec<(u32, String)> 
{
    let sys = System::new_all();
    let mut proc_info: Vec<(u32, String)> = Vec::new();

    for (pid, process)in sys.processes() {
        
        proc_info.push((pid.as_u32(), process.name().to_string()));
        if process.name().to_string() == "firefox" {
        	proc_mem_use(*pid, &sys);
        	println!("{:?}", process.status())
        }
    }
    return proc_info;
}

//unrelated info in the context of process
pub fn display_os_info() -> Vec<Option<String>>
{
	let mut sys = System::new();
	let mut os_info: Vec<Option<String>> = Vec::new();
	
	os_info.push(System::name());
	os_info.push(System::kernel_version());
	
	return os_info;
}


