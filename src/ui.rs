use std::collections::HashMap;
use ratatui::{prelude::*, widgets::{block::Title, *}};
use crate::{app::CurrentScreen, system_info::{cpu_data, network_data, process_data}};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TabWidget{
    pub tabs: [String; 3],
    pub selcted_tab: u32
}

impl TabWidget {
    pub fn new() -> TabWidget
    {
        TabWidget 
        {
            tabs: [String::from("Processes"), String::from("CPU"), String::from("Network")],
            selcted_tab: 0,
        }
    }

    pub fn update_seleceted_tab(&mut self)
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

pub struct FooterWidget{
    footer_text: String,
    style: Style,
}

impl FooterWidget{
    pub fn new() -> Self
    {
        Self 
        {
            footer_text: String::from("TAB => Change screens    |    'q' or 'Q' => Quit    |    Up & Down Arrow Keys => Scroll"),
            style: Style::new().blue(),
        }
    }


    pub fn update(&mut self, curr_screen: &CurrentScreen)
    {
        let cpu_and_net_text = String::from("TAB => Change screens    |    q  or 'Q' => Quit     ");
        let new_style = Style::new().bg(Color::Black).fg(Color::Green);
        match curr_screen
        {
            CurrentScreen::Cpu | CurrentScreen::Network => 
            {
                self.footer_text = cpu_and_net_text;
                self.style = new_style;
            }
            CurrentScreen::ProcessInfo =>
            {
                self.footer_text = String::from("TAB => Change screens    |    'q' or 'Q' => Quit    |    Up & Down Arrow Keys => Scroll");
                self.style = Style::new().bg(Color::Black).fg(Color::Blue);
            }
        }
    }

    
}

impl <'a> Widget for &'a FooterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = &self.footer_text;
        Paragraph::new(text.as_str())
            .alignment(Alignment::Center)
            .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(self.style))
            .render(area, buf)
    }
}

pub struct ProcessesScreen{
    //curr_screen: &'a CurrentScreen,
    screen_info: Vec<process_data::Process>,
    pub state: TableState,
    pub selected: Option<usize>
}

impl ProcessesScreen {
    const DEFAULT_SELECTION: usize = 0;
    pub fn new() -> ProcessesScreen
    {
        ProcessesScreen{
            screen_info: process_data::Processes::new().all_procs,
            state: TableState::default(),
            selected: Some(Self::DEFAULT_SELECTION),
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
            Color::White,   
            Color::Black,   
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

pub struct CpuScreen
{
    cpu_info: (f32, usize, u64, String),
    ram_info: HashMap<String, String>,
    sys_info: HashMap<String, String>,
}

impl CpuScreen
{
    pub fn new() -> Self
    {
        Self
        {
            cpu_info: cpu_data::fetch_cpu_info(),
            ram_info: cpu_data::fetch_ram_info(),
            sys_info: cpu_data::fetch_sys_info()
        }
    }

    pub fn render_widgets(&self, areas: [Rect; 3], buf: &mut Buffer)
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
            ["OS Name: ".to_string() + &info_data["OS Name"], 
            "OS Version: ".to_string() + &info_data["OS Ver."], 
            "Kernel Version: ".to_string() + &info_data["Kernel Ver."]]
        ),
        Row::new(
            ["Host Name: ".to_string() + &info_data["Host Name"], 
            "Uptime (s): ".to_string() + &info_data["Uptime"], 
            "CPU Architecture: ".to_string() + &info_data["CPU Architecture"]]
        )];

        let widths = vec![
            Constraint::Fill(1), 
            Constraint::Fill(1),
            Constraint::Fill(1),];

        let style = Style::from((
            Color::White,   
            Color::Black,   
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

pub struct NetworkScreen
{
    mac_addrs: HashMap<String, Vec<String>>
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

    fn render_widgets(&self, areas: [Rect;2], buf: &mut Buffer)
    {
        let [info_ar, _graph_ar] = areas;
        self.render_net_info(info_ar, buf);
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

    pub fn render_interface_block(&self, info: (&String, &Vec<String>), mac_ar: Rect, buf: &mut Buffer) {
        Widget::render(List::new(info.1.clone())
            .block(Block::bordered().title(info.0.clone()).bold())
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true)
            .style(Style::new().bg(Color::Black).fg(Color::Green)),
            mac_ar, 
            buf)
    }

    /*
    * Not good enough
    fn render_net_graph(&self, area: Rect, buf: &mut Buffer)
    {
        let mut data = vec![];
        for n in &self.mac_addrs
        {
            data.push((n.0.as_str(), 2));
        }
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
     */
    
    
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