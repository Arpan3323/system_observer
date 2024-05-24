
//use sysinfo::Pid;

//static sys: sysinfo::System = System::new_all();
pub mod app_data{
    
    
    pub mod process_data
    {
        use sysinfo::{System};
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
                all_procs.sort_by(|a,b|b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
                all_procs
            }
    }
    }

    pub mod cpu_data
    {
        use std::{collections::HashMap, time::Duration};

        use sysinfo::{System, RefreshKind,CpuRefreshKind};
        /*
        pub struct cpu_w_data 
        {
            //RAM
            pub sys_info: HashMap<String, String>,
            pub avg_util_nums: (f32, usize),
        }
         */
        //info per cpu
        pub fn fetch_cpu_info() -> (f32, usize)
        {
            let mut s = System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
            );
            
            // Wait a bit because CPU usage is based on diff.
            std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

            s.refresh_cpu();
            let cpus = s.cpus();
            let mut avg_cpu_util: f32 = 0.0;
            let num_cpu: usize = cpus.len();

            for cpu in cpus
            {
                avg_cpu_util += cpu.cpu_usage();
            }
            
            //avg
            if avg_cpu_util > 0.0 && num_cpu > 0 
            {
                avg_cpu_util /= num_cpu as f32;
            }
            (avg_cpu_util, num_cpu)
        }




        //RAM, Kernel version, etc.
        pub fn fetch_sys_info()  -> HashMap<String, String>
        {
            let mut s = System::new_all();
            s.refresh_all();
            let mut sys_info_map = HashMap::new();
            let t_mem = s.total_memory() / 1000000;
            let u_mem = s.used_memory() / 1000000;

            sys_info_map.insert(String::from("t_mem"), t_mem.to_string());
            sys_info_map.insert(String::from("u_mem"), u_mem.to_string());
            sys_info_map.insert(String::from("sys_name"), System::name().expect("CPU Data: Error getting sys name"));
            sys_info_map.insert(String::from("kern_v"), System::kernel_version().expect("CPU Data: Error getting kern v"));
            sys_info_map.insert(String::from("os_v"), System::os_version().expect("CPU Data: Error getting os v"));
            sys_info_map.insert(String::from("host_name"),System::host_name().expect("CPU Data: Error getting host name"));
            //sys_info_map.insert(String::from("host_name"),System::global_cpu_info(&self));
            
            sys_info_map
        }

    }


}
