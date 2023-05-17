use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// 傳送訊息
    #[structopt()]
    Start {
        /// 設定視窗。
        #[structopt(short, long)]
        win: String,

        /// 設定訊息。
        #[structopt(short, long)]
        msg: String,

        /// 延遲執行秒數。
        #[structopt(short, long)]
        delay: Option<u64>,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(name = "auto send msg", about = "模擬鍵鼠傳送訊息")]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}
