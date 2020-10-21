use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
/// 远程服务器操作工具
enum Cli {
    /// 从远程服务器拉取文件
    From {
        /// 是否是文件夹
        #[structopt(short, long)]
        is_folder: bool,
        /// 远程文件或文件夹路径
        #[structopt(parse(from_os_str))]
        remote: Option<PathBuf>,
        /// 本地文件或文件夹路径
        #[structopt(parse(from_os_str))]
        local: PathBuf,
    },
    /// 发送文件到远程服务器
    To {
        /// 本地文件或文件夹路径
        #[structopt(parse(from_os_str))]
        local: PathBuf,
        /// 远程文件或文件夹路径
        #[structopt(parse(from_os_str))]
        remote: PathBuf,
    },
}

fn main() {
    // 解析运行参数
    let args = Cli::from_args();
    println!("cmd => {:?}", args);
}
