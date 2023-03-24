use enigo::*;
use std::{thread, time};
use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;

use windows::{
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*, core::{PCWSTR, HSTRING},
};

fn main() {
    let CommandLineArgs { msg, action, delay, win } = CommandLineArgs::from_args();

    match action {
        cli::Action::Start => {
            println!("開始動作");

            let delay_time = match delay {
                Some(delay_time) => time::Duration::from_secs(delay_time),
                None => time::Duration::from_secs(0),
            };

            let msg = match msg {
                Some(msg) => msg,
                None => {
                    println!("必須有msg");
                    return;
                }
            };

            let win = match win {
                Some(win) => win,
                None => {
                    println!("必須輸入視窗");
                    return;
                }
            };

            thread::sleep(delay_time);

            let win_seded: BOOL;
            
            let win = HSTRING::from(&win).as_wide().as_ptr();
            let win = PCWSTR(win as _);

            unsafe {
                let win = FindWindowW(None, win);
                println!("{:#?}", win.0 > 0);
                
                win_seded = SetForegroundWindow(win);
                
            }

            let not_set_win = BOOL(0);
            if win_seded != not_set_win {
                let mut enigo = Enigo::new();

                enigo.key_sequence(&msg);

                enigo.key_click(Key::Return);

                println!("完成動作");
            }
            else{
                println!("請指定正確的視窗");
            }
        }
    }
}
