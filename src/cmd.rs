use std::ops::Add;

use clap::Parser;
use regex::Regex;

#[warn(dead_code)]
#[derive(Debug, Parser)]
#[command(name = "simulation",version, about, long_about = None)]
pub struct Simulation {
    #[arg(long, default_value_t = false, help = "是否 docker 环境")]
    pub docker: bool,

    #[arg(long, short, default_value_t = 10, help = "刷新间隔")]
    pub refresh: u8,

    #[command(subcommand)]
    pub sub_command: Options,
}

#[derive(Parser, Debug)]
pub enum Options {
    #[command(name = "net")]
    Network(NetworkOpts),

    #[command(name = "cpu")]
    CPU,

    #[command(name = "io")]
    IO(IoOpts),

    #[command(name = "mem")]
    Memory(MemoryOpts),
}

#[derive(Parser, Debug)]
pub struct MemoryOpts {
    #[arg(long, short, value_parser = value_parser_format)]
    pub num: String,
}

#[derive(Parser, Debug)]
pub struct IoOpts {
    #[arg(long, value_parser = value_parser_format)]
    pub num: String,
}

#[derive(Parser, Debug)]
pub struct NetworkOpts {
    #[arg(long, value_parser = value_parser_format)]
    pub num: String,

    #[arg(long, default_value_t = true)]
    pub dynamic: bool,
}

/**
 * 参数正则匹配
 */
fn value_parser_format(value: &str) -> Result<String, String> {
    let pattern = r"^[0-9]{1,}([kKmMGg%])$";

    let re = Regex::new(pattern).expect("创建正则失败 !");

    if re.captures(value).is_some() {
        Ok(value.into())
    } else {
        Err(String::add("参数正则匹配错误：".into(), value))
    }
}
