mod processes;
fn main()
{
	let proc_list = processes::Processes::new();
        for i in proc_list.all_procs
        {
            println!("{:?}", i)
        }
	/*
	let mut s = System::new_with_specifics(
	    RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
	);

	// Wait a bit because CPU usage is based on diff.
	std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
	// Refresh CPUs again.
	s.refresh_cpu();
	println!("{}", s.cpus().len());
	for cpu in s.cpus() {
	    println!("{}%", cpu.cpu_usage());
	}
	
	println!("{:?}", ThreadKind::clone(&ThreadKind::Kernel));
	*/
	
}
