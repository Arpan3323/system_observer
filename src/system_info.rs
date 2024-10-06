pub mod process_data
{
    use sysinfo::{Pid, System};
    #[derive(Debug)]
    pub struct Process
    {
        pub name: String,
        pub pid: Pid,
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

        pub fn kill_proc(&mut self, selected_table_index: usize)
        {
            //use sysinfo::{Pid, System};

            let s = System::new_all();
            if let Some(process) = s.process(self.all_procs[selected_table_index].pid) 
            {
                process.kill();
            }

            self.all_procs = Self::get_all_procs(); //imitate a refresh
        }

        fn get_all_procs() -> Vec<Process> 
        {
            let mut sys = System::new_all();
            //dividing cpu usage per proc by number of cpus to get a val b/w 0% to 100&
            let cpu_num = sys.cpus().len() as f32;
            let mut all_procs: Vec<Process> = Vec::new();
            sys.refresh_all();
            sys.refresh_all();
            for (pid, process)in sys.processes() 
            {
                if process.name().to_string() != "system-observer" &&
                process.name().to_string() != "system_observer"
                {
                    let curr_proc = Process {
                        name: process.name().to_string(),
                        pid: pid.to_owned(),
                        status: process.status().to_string(),
                        memory_usage: process.memory() / 1000000,
                        cpu_usage: process.cpu_usage() / cpu_num,
                    };
                    all_procs.push(curr_proc);
                }
            }
            
            //all_procs.sort_by(|a,b|b.cmp(a));
            all_procs.sort_by(|a,b|b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
            all_procs
        }


}
}

pub mod cpu_data
{
    use std::collections::HashMap;

    use sysinfo::{System, RefreshKind,CpuRefreshKind};
    //info per cpu
    pub fn fetch_cpu_info() -> (f32, usize, u64, String)
    {
        let mut s = System::new_with_specifics(
            RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
        );
        s.refresh_cpu();
        // Wait a bit because CPU usage is based on diff.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        s.refresh_cpu();
        let cpus = s.cpus();
        let num_cpu: usize = cpus.len();

        let mut avg_cpu_util: f32 = 0.0;
        let mut avg_freq: u64 = 0;
        let mut cpu_brand: String = String::new();
        for cpu in cpus
        {
            avg_cpu_util += cpu.cpu_usage();
            avg_freq += cpu.frequency();
            if cpu_brand.is_empty() {
                cpu_brand.push_str(cpu.brand());
            }
        }
        
        //avg
        if num_cpu > 0 && avg_freq > 0 
        {
            avg_freq /= num_cpu as u64;
        }
        
        (avg_cpu_util, num_cpu, avg_freq, cpu_brand)
    }




    //RAM, Kernel version, etc.
    pub fn fetch_ram_info()  -> HashMap<String, String>
    {
        let mut s = System::new_all();
        s.refresh_all();
        let mut ram_info = HashMap::new();
        let t_mem = s.total_memory() / 1000000;
        let u_mem = s.used_memory() / 1000000;
        
        ram_info.insert(String::from("t_mem"), t_mem.to_string());
        ram_info.insert(String::from("u_mem"), u_mem.to_string());
        
        ram_info
    }

    pub fn fetch_sys_info() -> HashMap<String, String>
    {
        let mut sys_info_map = HashMap::new();

        sys_info_map.insert(String::from("OS Name"), System::name().expect("CPU Data: Error getting sys name"));
        sys_info_map.insert(String::from("Host Name"),System::host_name().expect("CPU Data: Error getting host name"));
        sys_info_map.insert(String::from("Kernel Ver."), System::kernel_version().expect("CPU Data: Error getting kern v"));
        sys_info_map.insert(String::from("OS Ver."), System::os_version().expect("CPU Data: Error getting os v"));
        sys_info_map.insert(String::from("Uptime"),System::uptime().to_string());
        sys_info_map.insert(String::from("CPU Architecture"),System::cpu_arch().expect("CPU Data: Error getting arch"));

        sys_info_map
    }

}

pub mod network_data 
{
    use std::collections::HashMap;

    use sysinfo::Networks;

    pub fn fetch_macs() -> HashMap<String, Vec<String>>
    {
        let mut res = HashMap::new();
        let networks = Networks::new_with_refreshed_list();

        for (interface_n, network) in &networks
        {
            let interface_name = "Interface Name: ".to_string() + interface_n;

            let mac_addr = "MAC Address: ".to_string() + &network.mac_address().to_string();
            let egress = "Total Egress (bytes): ".to_string() + &network.total_transmitted().to_string();
            let ingress = "Total Ingress (bytes): ".to_string() + &network.total_received().to_string();
            let pack_out = "Total Packets Out: ".to_string() + &network.total_packets_transmitted().to_string();
            let pack_in = "Total Packets In: ".to_string() + &network.total_packets_received().to_string();

            let interface_data = vec![
                mac_addr, egress, ingress, pack_out, pack_in];
            res.insert(interface_name, interface_data );
        }
        res
    }

}

#[cfg(test)]
mod tests {
    /*
    * Happy path tests: starts with 0
    * Sad path tests: starts with 1
    * Evil path tests: start with 9
    */
    #[test]
    fn test101_exclude_app_name() {
        use crate::system_info::process_data::Processes;
        let result = Processes::new();
        for proc in result.all_procs
        {
            assert_ne!(proc.name, "system_observer");
            assert_ne!(proc.name, "system-observer");
        }
    }
}