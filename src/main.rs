use enigo::*;
use std::{thread, time};

fn main() {
    println!("開始動作");

    let five_s = time::Duration::from_secs(5);

    thread::sleep(five_s);

    let mut enigo = Enigo::new();
    enigo.mouse_move_to(1300, 1100);
    enigo.mouse_click(MouseButton::Left);
    
    enigo.key_sequence("測試一下");
    enigo.mouse_move_to(1461, 1100);
    enigo.mouse_click(MouseButton::Left);


    println!("完成動作");
}
