use std::{collections::HashMap, io::{stdout, Result}};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::{block::Title, *}};
use crate::processes::backend::{self, cpu_data, network_data};
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
    net_screen: NetworkScreen,
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
            net_screen: NetworkScreen::new(),
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
                self.net_screen.render(screen_ar, buf)
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
    screen_info: Vec<backend::process_data::Process>,
    state: TableState,
}

impl ProcessesScreen {
    fn new() -> ProcessesScreen
    {
        ProcessesScreen{
            //curr_screen: &App::new().current_screen,
            screen_info: backend::process_data::Processes::new().all_procs,
            state: TableState::default(),
        }
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

        let widths = vec![
            Constraint::Percentage(30), 
            Constraint::Percentage(10), 
            Constraint::Percentage(10), 
            Constraint::Percentage(20), 
            Constraint::Percentage(20)];

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

    }
}

struct CpuScreen
{
    cpu_info: (f32, usize, u64, String),
    ram_info: HashMap<String, String>,
    sys_info: HashMap<String, String>,
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
            ram_info: cpu_data::fetch_ram_info(),
            sys_info: cpu_data::fetch_sys_info()
        }
    }

    fn render_widgets(&self, areas: [Rect; 3], buf: &mut Buffer)
    {
        let data = Self::new();

        //ram data
        let ram_data = [&data.ram_info["t_mem"], &data.ram_info["u_mem"]];

        //info data
        let info_data = data.sys_info;
        
        //cpu util bar
        let [cpu_ar, ram_ar, info_ar] = areas;

        //render CPU bar
        let cpu_block = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("CPU Utilization (%)").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black).fg(Color::White));
        let cpu_util_bar = cpu_block.inner(cpu_ar);
        cpu_block.render(cpu_ar, buf);
        self.render_cpu_bar(cpu_util_bar, buf, data.cpu_info);
        
        //render RAM bar
        let ram_block = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("RAM Utilization (%)").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black).fg(Color::White));
        let ram_util_bar = ram_block.inner(ram_ar);
        ram_block.render(ram_ar, buf);
        self.render_ram_bar(ram_util_bar, buf, ram_data);

        //render system info
        let info_block = Block::new()
            .borders(Borders::ALL)
            .title(Title::from("System Info").alignment(Alignment::Center))
            .style(Style::new().bg(Color::Black).fg(Color::Blue));
        let info_block_cont_ar = info_block.inner(info_ar);
        info_block.render(info_ar, buf);
        self.render_info_cont(info_block_cont_ar, buf, info_data)
    }

    fn render_cpu_bar(&self, area: Rect, buf: &mut Buffer, util_nums: (f32, usize, u64, String))
    {
        //let time = time::Instant
        //let util_nums = cpu_w_data::new().avg_util_nums;
        
        
        let cpu_util = util_nums.0 as u16;
        let cpu_num = util_nums.1.to_string();
        let avg_freq = util_nums.2.to_string();
        let brand_name = util_nums.3;

        Gauge::default()
            .block(
                Block::bordered()
                .title(Title::from(String::from("No. of Cores: ") + &cpu_num + "     Avg. Frequency (MHz): " + &avg_freq)
                    .alignment(Alignment::Left)
                )
                .title(Title::from(brand_name)
                    .alignment(Alignment::Right)
                )
            )
            .gauge_style(
                Style::default()
                    .fg(Color::Green)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(cpu_util)
            .bold()
            .render(area, buf);
    }

    fn render_ram_bar(&self, area: Rect, buf: &mut Buffer, ram_data: [&String; 2])
    {
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
            .bold()
            .render(area, buf);
    }

    fn render_info_cont(&self, area: Rect, buf: &mut Buffer, info_data: HashMap<String, String>)
    {
        let rows = vec![Row::new(
            ["OS Name: ".to_string() + &info_data["OS Name"], "OS Version: ".to_string() + &info_data["OS Ver."], "Kernel Version: ".to_string() + &info_data["Kernel Ver."]]
        ),
        Row::new(
            ["Host Name: ".to_string() + &info_data["Host Name"], "Uptime (s): ".to_string() + &info_data["Uptime"], "CPU Architecture: ".to_string() + &info_data["CPU Architecture"]]
        )];

        let widths = vec![
            Constraint::Fill(1), 
            Constraint::Fill(1),
            Constraint::Fill(1),];

        let style = Style::from((
            Color::White,   //text
            Color::Black,   //bg
            Modifier::BOLD,
            ));

        Widget::render(
            Table::new(rows, widths)
                .block(Block::default().borders(Borders::ALL).style(Color::Magenta))
                //.header(headers)
                .highlight_style(Style::new().red())
                .style(style),
            area,
            buf,
        );
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

struct NetworkScreen
{
    mac_addrs: HashMap<String, Vec<String>>,
    //max_interfaces: usize

}

impl NetworkScreen
{
    const MAX_INTERFACES: usize = 4;

    pub fn new() -> Self
    {
        Self
        {
            mac_addrs: network_data::fetch_macs(),
        }
    }

    pub fn render_widgets(&self, areas: [Rect;2], buf: &mut Buffer)
    {
        let [info_ar, graph_ar] = areas;
        self.render_net_info(info_ar, buf);
        //self.render_net_graph(graph_ar, buf)
        //self.render(area, buf)
        
    }
        
    fn render_net_info(&self, info_ar: Rect, buf: &mut Buffer) 
    {
        let info_block = Block::bordered()
            .style(Style::new().bg(Color::Black).fg(Color::Blue));
        

        let inner_ar = info_block.inner(info_ar);

        let info_block_layout = Layout::horizontal(
            [Constraint::Fill(1), Constraint::Fill(1),
            Constraint::Fill(1), Constraint::Fill(1)]
        );
        let inter_info_ar: [Rect; 4] = info_block_layout.areas(inner_ar);
        info_block.render(info_ar, buf);
        let mut i = 0;
        for n in &self.mac_addrs
        {
            if i < Self::MAX_INTERFACES
            {
                self.render_interface_block(n, inter_info_ar[i], buf);
                i += 1;
            }
        }
        
    }

    fn render_interface_block(&self, info: (&String, &Vec<String>), mac_ar: Rect, buf: &mut Buffer) {
        Widget::render(List::new(info.1.clone())
            .block(Block::bordered().title(info.0.clone()).bold())
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .style(Style::new().bg(Color::Black).fg(Color::Green)),
            mac_ar, 
            buf)
    }

    fn render_net_graph(&self, area: Rect, buf: &mut Buffer)
    {
        let mut data = vec![];
        for n in &self.mac_addrs
        {
            data.push((n.0.as_str(), 2));
        }
        //let (name, info) = &self.mac_addrs;
        BarChart::default()
            .block(Block::bordered().title("BarChart"))
            .bar_width(3)
            .bar_gap(5)
            .group_gap(10)
            .bar_style(Style::new().yellow().on_red())
            .value_style(Style::new().red().bold())
            .label_style(Style::new().white())
            .data(&data)
            .data(BarGroup::default().bars(&[Bar::default().value(10), Bar::default().value(20)]))
            .max(4)
            .render(area, buf);
    }
    
    
}



impl <'a> Widget for &'a NetworkScreen
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let layout = Layout::vertical(
            [     
                Constraint::Percentage(100),
                Constraint::Fill(1)
                ]);

        self.render_widgets(layout.areas(area), buf);
        

    }
}