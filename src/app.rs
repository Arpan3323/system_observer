use std::{io::{self, stdout, Result}, os::unix::process};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::processes::*;

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
    screen: Screen,
    footer: FooterWidget,
	app_state: AppState,
}


impl App {
	pub fn new() -> App {
		App {
			tab: TabWidget::new(),
			current_screen: CurrentScreen::ProcessInfo,
            screen: Screen::,
            footer: FooterWidget::new(),
			app_state: AppState::Running,
			//screen_info: processes::Processes.
		}
	}

    pub fn run(&mut self) -> Result<()>
    {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        //let mut should_quit = false;

        while self.app_state != AppState::Exiting 
        {
            
            terminal.draw(|frame| 
                {
                    self.render(frame.size(), frame.buffer_mut());
                    //frame.render_widget(self, frame.size())
                    //App::render(*self, frame.size(), frame.buffer_mut());
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
        match key.code
        {
            //KeyCode::Tab => self.change_tab(),
            KeyCode::Char('q') => self.quit_app(),
            //KeyCode::Tab => self.change_tab(),
            _ => (),
        };
    }

    fn quit_app(&mut self) 
    {
        self.app_state = AppState::Exiting;
    }
    
}

impl <'a> Widget for &'a App
{
    fn render(self, area: Rect, buf: &mut Buffer)
    {
        let app_layout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Percentage(90),
            Constraint::Length(3),
        ]);
                        
        let [tab_ar, screen_ar, foot_ar] = app_layout.areas(area);
        //Block::new().style(Style::new().bg(Color::Rgb(16, 24, 48))).render(area, buf);
        self.tab.render(tab_ar, buf);
        Block::bordered().render(screen_ar, buf);
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
}

impl TabWidget {
    fn new() -> TabWidget
    {
        TabWidget 
        {
            tabs: [String::from("Processes"), String::from("CPU"), String::from("Network")],
        }
    }
}

impl <'a> Widget for &'a TabWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        //let titles = SelectedTab::iter().map(SelectedTab::title);
        //self.tabs.push(String::from("Processes"));
        //select_title = titles as usize;
        //let highlight_style = (Color::default(), );
        //let selected_tab_index = self.selected_tab as usize;
        let titles = &self.tabs;
        Tabs::new(titles.to_vec())
            .highlight_style(Style::new().fg(Color::Blue).bg(Color::White))
            .block(block::Block::new().borders(Borders::ALL).border_style(Style::new().bg(Color::Red)))
            .select(0)
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


pub struct Screen{
    //curr_screen: &'a CurrentScreen,
    screen_info: Processes
}

impl Screen {
    fn new(screen_type: CurrentScreen) -> Screen
    {
        SCreen
        screen_info: Processes::new()
        
    }
    
}
