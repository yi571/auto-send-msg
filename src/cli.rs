use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// 傳送訊息
    #[structopt()]
    Start
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "auto send msg",
    about = "模擬鍵鼠傳送訊息"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// 設定input x軸。
    #[structopt(short = "x", long = "x_axis")]
    pub input_x_axis: Option<i32>,

    /// 設定input y軸。
    #[structopt(short = "y", long = "y_axis")]
    pub input_y_axis: Option<i32>,
    
    /// 設定訊息。
    #[structopt(short, long)]
    pub msg: Option<String>,

    /// 延遲執行秒數。
    #[structopt(short, long)]
    pub delay: Option<u64>,
}