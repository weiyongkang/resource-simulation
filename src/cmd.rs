use std::{ops::Add, path::PathBuf};

use clap::Parser;
use regex::Regex;

#[warn(dead_code)]
#[derive(Debug, Parser)]
#[command(name = "simulation",version, about, long_about = None)]
pub struct Simulation {
	#[arg(long, default_value_t = false, help = "是否 docker 环境")]
	pub docker: bool,

	#[arg(long, short, default_value_t = 5, help = "刷新间隔")]
	pub refresh: u8,

	#[command(subcommand)]
	pub sub_command: Options,
}

#[derive(Parser, Debug)]
pub enum Options {
	#[command(name = "net", about = "网络IO")]
	Network(NetworkOpts),

	#[command(name = "cpu", about = "CPU 模拟")]
	CPU,

	#[command(name = "io", about = "文件 IO 模拟")]
	IO(IoOpts),

	#[command(name = "mem", about = "内存模拟")]
	Memory(MemoryOpts),

	#[command(name = "file", about = "新建文件，占用空间")]
	File(FileOpts),
}

#[derive(Parser, Debug)]
pub struct MemoryOpts {
	#[arg(long, short, value_parser = verify_value_file_unit)]
	pub num: String,
}

#[derive(Parser, Debug)]
pub struct IoOpts {
	#[arg(long, short, value_parser = verify_value_file_unit, help = "读取流大小")]
	pub input: String,

	#[arg(long, short, value_parser = verify_value_file_unit, help = "写入流大小")]
	pub output: String,

	#[arg(long, short,default_value = ".", value_parser = verify_value_file_dir, help = "文件路径")]
	pub dirname: String,

	#[arg(long, short, default_value = "resource.simulation", help = "文件名")]
	pub filename: String,
}

#[derive(Parser, Debug)]
pub struct FileOpts {
	#[arg(long, short, default_value = "resource.create", help = "文件名")]
	pub filename: String,

	#[arg(long, default_value_t = 1, help = "文件数量")]
	pub filecount: u8,

	#[arg(long, short,default_value = ".", value_parser = verify_value_file_dir, help = "生成文件名路径")]
	pub dirname: String,

	#[arg(long, short, value_parser = verify_value_file_unit, help = "文件大小")]
	pub num: String,
}

#[derive(Parser, Debug)]
pub struct NetworkOpts {
	#[arg(long, value_parser = verify_value_file_unit)]
	pub num: String,

	#[arg(long, default_value_t = true)]
	pub dynamic: bool,
}

/**
 * 校验 值 是否合法
 */
fn verify_value_file_unit(value: &str) -> Result<String, String> {
	let pattern = r"^[0-9]{1,}([bkKmMGg%])$";

	let re = Regex::new(pattern).expect("创建正则失败 !");

	if re.captures(value).is_some() {
		Ok(value.into())
	} else {
		Err(String::add("参数正则匹配错误：".into(), value))
	}
}

/**
 * 校验目录路径是否存在
 */
fn verify_value_file_dir(value: &str) -> Result<String, String> {
	let path = PathBuf::from(value);
	if !path.is_dir() {
		Err(format!("文件路径: {} 不存在!!", value))
	} else {
		Ok(value.into())
	}
}
