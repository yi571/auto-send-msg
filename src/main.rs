use enigo::*;
use std::{thread, time};
use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;

use windows::{
    core::{HSTRING, PCWSTR},
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    let command_line_args = CommandLineArgs::from_args();

    match command_line_args.action {
        cli::Action::Start {
            msg,
            delay,
            win,
        } => {
            println!("開始動作");

            let delay_time = match delay {
                Some(delay_time) => time::Duration::from_secs(delay_time),
                None => time::Duration::from_secs(0),
            };


            thread::sleep(delay_time);

            let win_seded: BOOL;

            let win = HSTRING::from(&win).as_wide().as_ptr();
            let win = PCWSTR(win as _);

            unsafe {
                let win = FindWindowW(None, win);
                println!("{:#?}", win.0);

                win_seded = SetForegroundWindow(win);
            }

            let not_set_win = BOOL(0);
            if win_seded != not_set_win {
                let mut enigo = Enigo::new();

                enigo.key_sequence(&msg);

                enigo.key_click(Key::Return);

                println!("完成動作");
            } else {
                println!("請指定正確的視窗");
            }
        }
    }
}
