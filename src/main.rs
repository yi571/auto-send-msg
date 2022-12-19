use enigo::*;
use std::{thread, time};
use structopt::StructOpt;

mod cli;
use cli::CommandLineArgs;

fn main() {
    let CommandLineArgs {
        input_x_axis,
        input_y_axis,
        msg,
        action,
        delay,
    } = CommandLineArgs::from_args();

    match action {
        cli::Action::Start => {
            println!("開始動作");

            let delay_time = match delay {
                Some(delay_time) => time::Duration::from_secs(delay_time),
                None => time::Duration::from_secs(0),
            };

            let input_x_axis = match input_x_axis {
                Some(input_x_axis) => input_x_axis,
                None => {
                    println!("必須有inputx");
                    return;
                }
            };

            let input_y_axis = match input_y_axis {
                Some(input_y_axis) => input_y_axis,
                None => {
                    println!("必須有inputy");
                    return;
                }
            };

            let msg = match msg {
                Some(msg) => msg,
                None => {
                    println!("必須有msg");
                    return;
                }
            };


            thread::sleep(delay_time);

            let mut enigo = Enigo::new();
            //enigo.mouse_move_to(1300, 1100);
            enigo.mouse_move_to(input_x_axis, input_y_axis);
            enigo.mouse_click(MouseButton::Left);

            enigo.key_sequence(&msg);
            //enigo.mouse_move_to(1461, 1100);
            // enigo.mouse_move_to(button_x_axis, button_y_axis);
            // enigo.mouse_click(MouseButton::Left);

            enigo.key_click(Key::Return);

            println!("完成動作");
        }
    }
}
