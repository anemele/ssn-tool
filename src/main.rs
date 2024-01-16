use clap::Parser;
use ssn::{check_gen2, cvt1to2, cvt2to1};

#[derive(Parser)]
#[clap(
    name = "ssn",
    version,
    author,
    about = "身份证号工具",
    long_about = "\
中华人民共和国身份证号工具，提供以下功能：

    一代和二代身份证号相互转换
    校验二代身份证号"
)]
enum Cli {
    /// 一代身份证号转换为二代身份证号
    #[clap(name = "1to2")]
    Cvt1to2 {
        #[arg(help = "一代身份证号")]
        id: String,
        #[arg(help = "出生年份前两位")]
        year2: String,
    },
    /// 二代身份证号转换为一代身份证号
    #[clap(name = "2to1")]
    Cvt2to1 {
        #[arg(help = "二代身份证号")]
        id: String,
    },
    /// 校验二代身份证号
    Check {
        #[arg(help = "二代身份证号")]
        id: String,
    },
}

fn main() {
    match Cli::parse() {
        Cli::Cvt1to2 { id, year2 } => {
            // println!("{},{}", id, year2);
            if let Some(ret) = cvt1to2(&id, &year2) {
                println!("{} -> {}", id, ret)
            }
        }
        Cli::Cvt2to1 { id } => {
            // println!("{}", id);
            if let Some(ret) = cvt2to1(&id) {
                println!("{} -> {}", id, ret)
            }
        }
        Cli::Check { id } => {
            // println!("{}", id);
            if check_gen2(&id) {
                println!("ok   {id}")
            } else {
                println!("bad  {id}")
            }
        }
    }
}
