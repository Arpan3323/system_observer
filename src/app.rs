use std::io::{stdout, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use crate::ui::*;

//mod processes;
pub enum CurrentScreen
{
	ProcessInfo,
	Cpu,
	Network,
}
#[derive(PartialEq)]
pub enum AppState
{
	Running,
	Exiting,
}

pub struct App 
{
	tab: TabWidget,
	current_screen: CurrentScreen,
    process_screen: ProcessesScreen,
    process_screen_state: TableState,
    footer: FooterWidget,
	app_state: AppState,
    cpu_screen: CpuScreen,
    net_screen: NetworkScreen,
}


impl App {
	pub fn new() -> App{
		App 
        {
			tab: TabWidget::new(),
			current_screen: CurrentScreen::ProcessInfo,  
            process_screen: ProcessesScreen::new(),
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
                    KeyCode::Char('q' | 'Q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    KeyCode::Down => self.move_down(),
                    KeyCode::Up => self.move_up(),
                    KeyCode::Char('k' | 'K') => self.process_screen.kill_by_pid(),
                    _ => {}
                }
            },
            CurrentScreen::Cpu => {
                match key.code
                {
                    KeyCode::Char('q' | 'Q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    _ => {}
                }
            },
            CurrentScreen::Network => {
                match key.code
                {
                    KeyCode::Char('q' | 'Q') => self.quit_app(),
                    KeyCode::Tab => self.change_tab(),
                    _ => {}
                }
            },
            
        }
    }

    fn move_up(&mut self) {
        let selected = self.process_screen_state.selected();
        match selected
        {
            Some(index) => if index != 0 {self.process_screen_state.select(Some(index - 1))},
            None => self.process_screen_state.select(Some(0))
        }
        self.process_screen.selected = self.process_screen_state.selected();
    }
    
    fn move_down(&mut self) 
    {
        let selected = self.process_screen_state.selected();
        match selected
        {
            Some(index) => self.process_screen_state.select(Some(index + 1)),
            None => self.process_screen_state.select(Some(0))
        }
        self.process_screen.selected = self.process_screen_state.selected();
        
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
            {
                self.footer.update(&CurrentScreen::ProcessInfo);
                self.process_screen.render(screen_ar, buf, &mut self.process_screen_state)
            }
            CurrentScreen::Cpu => 
            {
                self.footer.update(&CurrentScreen::Cpu);
                self.cpu_screen.render(screen_ar, buf)
            }
            CurrentScreen::Network => 
            {
                self.footer.update(&CurrentScreen::Network);
                //self.cpu_screen.render(screen_ar, buf)
                self.net_screen.render(screen_ar, buf)
            }
                //self.net_screen.render(screen_ar, buf)
        }
        self.footer.render(foot_ar, buf);
        //let current_screen_clone = &self.current_screen;
        //self.footer.update(&self.current_screen);
    }
}