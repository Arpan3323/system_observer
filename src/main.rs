use std::io::{self, stdout, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
mod processes;

/*
 * enter the Alternate Screen when starting and leave it when exiting 
 * and also enable raw mode to disable line buffering and enable reading key events
 */
fn main() -> Result<()>
{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())





    //let proc_list = processes::Processes::new();
    //for i in proc_list.all_procs
    //{
    //  println!("{:?}", i)
    //}
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


fn handle_events() -> Result<bool>
{
    let timeout = std::time::Duration::from_millis(50);
    
    if event::poll(timeout)?
    {
        if let Event::Key(key) = event::read()?
        {
            if key.kind == event::KeyEventKind::Press
            {
                match key.code
                {
                    KeyCode::Char('q') => return Ok(true),
                    _ => return Ok(false),
                }
            }
        }
    }

    Ok(false)

}


fn ui(frame: &mut Frame)
{

    let layout = Layout::new(
    Direction::Horizontal,
    [Constraint::Percentage(50)],
    ).split(Rect::new(0,0,100,100));
    
    let style = Style::from((
            Color::White,   //text
            Color::Black,   //bg
            Modifier::BOLD,
            ));

    let proc_list = processes::Processes::new().all_procs;
    let mut rows = Vec::new();
    rows.push(Row::new(["Name", "PID", "Status", "Memory", "CPU"]).style(Style::new().red()));
    for i in proc_list{
        rows.push(Row::new([
                i.name, 
                i.pid.to_string(), 
                i.status, 
                i.memory_usage.to_string(), 
                i.cpu_usage.to_string()
                ]));
    }


    frame.render_widget(Table::default()
        .rows(rows)
        .style(style),
        frame.size());
    /*
    frame.render_widget(
        Paragraph::new("Hello Human!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
            layout[0],
        );
    
     frame.render_widget(
        Paragraph::new("Hello Human!")
            .block(Block::default().title("Greeting").borders(Borders::ALL)),
            layout[1],
        );
    */
}

