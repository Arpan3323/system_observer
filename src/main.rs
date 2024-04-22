mod processes;
fn main()
{
	let all_procs: Vec<(u32, String)> = processes::get_all_procs();
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
