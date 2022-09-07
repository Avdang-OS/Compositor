#![allow(irrefutable_let_patterns)]

pub mod core;
mod consts;

use crate::consts as CONST;
pub(crate) use crate::config::Config;
pub(crate) use crate::core as Nadva;


mod grabs;
mod handler;
mod input;
mod state;
mod winit;

use slog::{
    Drain,
    Logger,
};

use smithay::reexports::{
    calloop::EventLoop,
    wayland_server::Display
};

pub use state::AvCompositor;

use std::error::Error;

pub struct CalloopData {
    state  : AvCompositor,
    display: Display<AvCompositor>,
}

pub mod config;



// mod compositor;
// use crate::compositor::winit::init_winit;

fn main() -> Result<(), Box<dyn Error>> {
    {
        let log: Logger = ::slog::Logger::root(::slog_stdlog::StdLog.fuse(), slog::o!());
        slog_stdlog::init()?;

        let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

        let mut display: Display<AvCompositor> = Display::new()?;
        let state      : AvCompositor          = AvCompositor::new(&mut event_loop, &mut display, log.clone());

        let mut data: CalloopData = CalloopData { state, display };

        crate::winit::init_winit(&mut event_loop, &mut data, log)?;

        let mut args = std::env::args().skip(1); //: impl Iterator<Item = String>
        let flag: Option<String> = args.next();
        let arg: Option<String>  = args.next();

        match (flag.as_deref(), arg) {
            (Some("-c") | Some("--command"), Some(command)) => {
                std::process::Command::new(command).spawn().ok();
            },
            _ => {
                std::process::Command::new("alacritty").spawn().ok();
            }
        }

        event_loop.run(None, &mut data, move |_| { /* The compositor is running */ })?;
    }

    let config = Config::from_file()
        .unwrap();
        // #![allow(unused_must_use)]
        // init_winit();
        let config = Config::from_file()?;
    
        println!("{config:?}");
        
        Ok(())
}
