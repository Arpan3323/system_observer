use std::{collections::HashMap, io::{stdout, Result}};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::{block::Title, *}};
use crate::processes::app_data::{self, cpu_data::{self}};
pub enum CurrentScreen{
	ProcessInfo,
	Cpu,
	Network,
}
#[derive(PartialEq)]
pub enum AppState{
	Running,
	Exiting,
}

pub struct App {
	tab: TabWidget,
	current_screen: CurrentScreen,
    screens: ProcessesScreen,
    process_screen_state: TableState,
    footer: FooterWidget,
	app_state: AppState,
    cpu_screen: CpuScreen,
}


impl App {
	pub async fn new() -> App{
		App {
			tab: TabWidget::new(),
			current_screen: CurrentScreen::ProcessInfo,
            screens: ProcessesScreen::new(),
            process_screen_state: TableState::default(),
            footer: FooterWidget::new(),
			app_state: AppState::Running,
            cpu_screen: CpuScreen::new(),
		}
	}

    pub fn run(&mut self) -> Result<()>
    {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        while self.app_state != AppState::Exiting 
        {
            
            terminal.draw(|frame| 
                {
                    self.render(frame.size(), frame.buffer_mut());
                }

            )?;
            self.handle_events();
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
	
    }

    fn handle_events(&mut self)
    {
        let timeout = std::time::Duration::from_millis(50);
        
        if event::poll(timeout).expect("Error: Event poll time out")
        {
            if let Event::Key(key) = event::read().expect("Error: Reading key failed")
            {
                if key.kind == event::KeyEventKind::Press
                {
                    self.handle_key_press(key);
                }
            }
        }

    }

    fn handle_key_press(&mut self, key: KeyEvent)
    {
        match self.current_screen
        {
            CurrentScreen::ProcessInfo => 
            {
                match key.code
                {
                    KeyCode::Char('q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    KeyCode::Down => self.process_screen_state.select(Some(self.process_screen_state.selected().unwrap_or(0) + 1)),
                    KeyCode::Up => self.process_screen_state.select(Some(self.process_screen_state.selected().unwrap_or(0) - 1)),
                    _ => {}
                }
            },
            CurrentScreen::Cpu => {
                match key.code
                {
                    KeyCode::Char('q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    _ => {}
                }
            },
            CurrentScreen::Network => {
                match key.code
                {
                    KeyCode::Char('q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    _ => {}
                }
            },
            
        }
    }

    fn quit_app(&mut self) 
    {
        self.app_state = AppState::Exiting;
    }

    fn change_tab(&mut self)
    {
        self.tab.update_seleceted_tab();
        self.current_screen = match self.tab.selcted_tab
        {
            0 => CurrentScreen::ProcessInfo,
            1 => CurrentScreen::Cpu,
            2 => CurrentScreen::Network,
            _ => CurrentScreen::ProcessInfo,
        };
    }
    
}

impl <'a> Widget for &'a mut App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let app_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(90),
            Constraint::Length(3),
        ]);
                        
        let [tab_ar, screen_ar, foot_ar] = app_layout.areas(area);
        self.tab.render(tab_ar, buf);
        match self.current_screen
        {
            CurrentScreen::ProcessInfo => 
                ProcessesScreen::new().render(screen_ar, buf, &mut self.process_screen_state),
            CurrentScreen::Cpu => 
                self.cpu_screen.render(screen_ar, buf),
            CurrentScreen::Network => 
                self.screens.render(screen_ar, buf, &mut self.process_screen_state),
        }
        self.footer.render(foot_ar, buf);
    }
}

/*
fn ui(frame: &mut Frame) {
    let app_layout = Layout::vertical([
        Constraint::Length(3),
        Constraint::Percentage(90),
        Constraint::Length(3),
     ]);
                    
    let [tab_ar, screen_ar, foot_ar] = app_layout.areas(frame.size());


    let proc_list = processes::Processes::new().all_procs;
    let mut rows = Vec::new();
    rows.push(Row::new(["Name", "PID", "Status", "Memory", "CPU"]).style(Style::new().red()));
    for i in proc_list
    {
        rows.push(Row::new([
                i.name, 
                i.pid.to_string(), 
                i.status, 
                i.memory_usage.to_string(), 
                i.cpu_usage.to_string()
                ]));
    }
            
                    
    let vertical = Layout::vertical([
        Constraint::Length(3),
            Constraint::Min(0),
        //Constraint::Percentage(90),
     ]);
                    
    let [tab_area, screen_area] = vertical.areas(frame.size());

             
    let custom_widget = TabWidget::new().render(tab_area, frame.buffer_mut());
    //render process list (Table) if current Tab is "Processes"
    let style = Style::from((
        Color::White,   //text
        Color::Black,   //bg
        Modifier::BOLD,
        ));
    frame.render_widget(Table::default()
    .rows(rows)
    .style(style),
    screen_area);
     
}
*/


#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TabWidget{
    tabs: [String; 3],
    selcted_tab: u32
}

impl TabWidget {
    fn new() -> TabWidget
    {
        TabWidget 
        {
            tabs: [String::from("Processes"), String::from("CPU"), String::from("Network")],
            selcted_tab: 0,
        }
    }

    fn update_seleceted_tab(&mut self)
    {
        self.selcted_tab = (self.selcted_tab + 1) % 3;
    }
}

impl <'a> Widget for &'a TabWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let titles = &self.tabs;
        Tabs::new(titles.to_vec())
            .highlight_style(Style::new().fg(Color::Blue).bg(Color::White))
            .block(block::Block::new().borders(Borders::ALL).border_style(Style::new().bg(Color::Red)))
            .select(self.selcted_tab as usize)
            .padding("    ", "    ")
            .divider("|")
            .render(area, buf);
    }
}

struct FooterWidget{
    ctrl_text: String,
    //is_pressed: bool,
    style: Style,
    //pressed_style: Option<Style>,
}

impl FooterWidget {
    fn new() -> FooterWidget
    {
        FooterWidget 
        {
            ctrl_text: String::from("Press TAB to change screens"),
            //is_pressed: false,
            style: Style::new().blue(),
            //pressed_style: None
        }
    }
}

impl <'a> Widget for &'a FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = &self.ctrl_text;
        Paragraph::new(text.as_str())
            .alignment(Alignment::Center)
            .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.style))
            .render(area, buf)
    }
}

//#[derive(Debug)]
pub struct ProcessesScreen{
    //curr_screen: &'a CurrentScreen,
    screen_info: Vec<app_data::process_data::Process>,
    state: TableState,
}

impl ProcessesScreen {
    fn new() -> ProcessesScreen
    {
        ProcessesScreen{
            //curr_screen: &App::new().current_screen,
            screen_info: app_data::process_data::Processes::new().all_procs,
            state: TableState::default(),
        }
        
    }

    fn next(&mut self)
    {
        self.state.select(Some(self.state.selected().unwrap_or(0) + 1));
    }

    fn previous(&mut self)
    {
        self.state.select(Some(self.state.selected().unwrap_or(0) - 1));
    }

    fn select(&mut self, index: usize)
    {
        self.state.select(Some(index));
    }

    fn unselect(&mut self)
    {
        self.state.select(None);
    }

    
}

impl <'a> StatefulWidget for &'a ProcessesScreen {
    type State = TableState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        //let render_rate = 1;
        let proc_list = &self.screen_info;
        let mut rows = Vec::new();
        let headers = Row::new(["Name", "PID", "Status", "Memory", "CPU"]).style(Style::new().red());



        for i in proc_list
        {
            rows.push(Row::new([
                    i.name.clone(), 
                    i.pid.to_string(), 
                    i.status.clone(), 
                    i.memory_usage.to_string(), 
                    i.cpu_usage.to_string()
                    ]));
        }
        let widths = vec![Constraint::Percentage(30), Constraint::Percentage(10), Constraint::Percentage(10), Constraint::Percentage(20), Constraint::Percentage(20)];
        let style = Style::from((
            Color::White,   //text
            Color::Black,   //bg
            Modifier::BOLD,
            ));
        StatefulWidget::render(
            Table::new(rows, widths)
                .block(Block::default().borders(Borders::ALL).title("Processes"))
                .header(headers)
                .highlight_style(Style::new().red())
                .style(style),
            area,
            buf,
            state,
        );
        //thread::sleep(time::Duration::from_millis(100));
    }
}

struct CpuScreen
{
    cpu_info: (f32, usize),
    sys_info: HashMap<String, String>
}

impl CpuScreen
{
    fn new() -> Self
    {
        //let data = cpu_data::cpu_w_data::new().await;
        //let c_cpu_data = &data.info_per_cpu;
        Self
        {
            cpu_info: cpu_data::fetch_cpu_info(),
            sys_info: cpu_data::fetch_sys_info()
        }
    }

    fn render_widgets(&self, areas: [Rect; 3], buf: &mut Buffer)
    {
        let data = Self::new();
        let ram_data = [&data.sys_info["t_mem"], &data.sys_info["u_mem"]];
        //cpu util bar
        let [cpu_ar, ram_ar, info_ar] = areas;
        let cpu_util_bar_ar = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("CPU Utilization (%)").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black));
        let cpu_util_bar = cpu_util_bar_ar.inner(cpu_ar);
        cpu_util_bar_ar.render(cpu_ar, buf);
        self.render_cpu_bar(cpu_util_bar, buf, data.cpu_info);
        
        //RAM util bar
        let ram_util_bar_ar = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("RAM Utilization (%)").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black));
        let ram_util_bar = ram_util_bar_ar.inner(ram_ar);
        ram_util_bar_ar.render(ram_ar, buf);
        self.render_ram_bar(ram_util_bar, buf, ram_data);

        let info = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("System Info").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black));
        
        info.render(info_ar, buf);
    }

    fn render_cpu_bar(&self, area: Rect, buf: &mut Buffer, util_nums: (f32, usize))
    {
        //let time = time::Instant
        //let util_nums = cpu_w_data::new().avg_util_nums;
        
        
        let cpu_util = util_nums.0 as u16;
        let cpu_num: String = util_nums.1.to_string();
        Gauge::default()
            .block(
                Block::bordered()
                .title(String::from("No. of Cores: ") + &cpu_num)
                .title_alignment(Alignment::Left)
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(cpu_util)
            .render(area, buf);
    }

    fn render_ram_bar(&self, area: Rect, buf: &mut Buffer, ram_data: [&String; 2])
    {
        //let time = time::Instant
        //let util_nums = cpu_w_data::new().avg_util_nums;
        
        
        let t_mem = ram_data[0];
        let u_mem = ram_data[1];
        let ram_util: f32 = (u_mem.clone().parse::<f32>().unwrap() / 
        t_mem.clone().parse::<f32>().unwrap()) * 100.00;
        let a_mem = (t_mem.clone().parse::<f32>().unwrap() - u_mem.clone().parse::<f32>().unwrap()).to_string();
        //let cpu_num: String = util_nums.1.to_string();
        Gauge::default()
            .block(Block::bordered().title(String::from("Used Ram (MB): ") 
            + u_mem + &String::from("    Total Ram (MB): ") + t_mem + &String::from("    Available Ram (MB): ") + &a_mem)
            .title_alignment(Alignment::Left))
            .gauge_style(
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(ram_util as u16)
            .render(area, buf);
    }


}

impl <'a> Widget for &'a CpuScreen
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        
        let layout = Layout::vertical(
            [
                Constraint::Percentage(40), 
                Constraint::Percentage(30),
                Constraint::Fill(1)
                ]);
        self.render_widgets(layout.areas(area), buf);
            
    }
}

struct Screens {
    proc_screen: ProcessesScreen,
    //cpu_screen: &'a mut CpuScreen,
    //network_screen: &'a mut NetworkScreen,
}

impl Screens {
    fn new() -> Screens {
        Screens {
            proc_screen: ProcessesScreen::new(),
            //cpu_screen,
            //network_screen,
        }
    }
}