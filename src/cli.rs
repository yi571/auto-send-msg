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

    /// 設定視窗。
    #[structopt(short, long)]
    pub win: Option<String>,

    /// 設定訊息。
    #[structopt(short, long)]
    pub msg: Option<String>,

    /// 延遲執行秒數。
    #[structopt(short, long)]
    pub delay: Option<u64>,
}