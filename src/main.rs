use enigo::*;
use std::{thread, time};
use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;

use windows::{
    w,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

fn main() {
    let CommandLineArgs { msg, action, delay } = CommandLineArgs::from_args();

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

            thread::sleep(delay_time);

            let win_seded: BOOL;

            unsafe {
                let win = FindWindowW(None, w!("XXX"));
                //println!("{:#?}", win);
                
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
