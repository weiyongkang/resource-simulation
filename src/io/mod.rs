use anyhow::Result;
use std::{
	fs::{File, OpenOptions},
	// hint::black_box,
	io::{Read, Seek, SeekFrom, Write},
	path::PathBuf,
	thread::sleep,
	time::{Duration, SystemTime},
};
use sysinfo::Disks;

use crate::{
	cmd::{FileOpts, IoOpts},
	tools::{
		number_format_conversion, number_format_to_string, random, string_to_number_format_u64,
	},
};

pub fn process(opts: &IoOpts, refresh: u8) -> Result<()> {
	// 获得绝对路径
	let curr_path: PathBuf = PathBuf::from(opts.dirname.as_str()).canonicalize().unwrap();

	if !curr_path.is_dir() {
		panic!("路径不存在: {}", opts.dirname);
	}

	let disks = Disks::new_with_refreshed_list();
	println!("{:?}", disks);
	// 获取 指定路径磁盘的信息
	let curr_disk = disks
		.into_iter()
		.filter(|disk| curr_path.starts_with(disk.mount_point().canonicalize().unwrap())) //
		.max_by(|x, y| x.mount_point().cmp(y.mount_point())) // 取得最大的那个挂载点
		.unwrap();

	let disk_total = curr_disk.total_space();
	let disk_available = curr_disk.available_space();

	println!(
		"磁盘挂载点为: {:?},总值为: {}, 可使用值为: {}",
		curr_disk.mount_point(),
		number_format_to_string(disk_total),
		number_format_to_string(disk_available)
	);

	let input_byte: u64 = match string_to_number_format_u64(&opts.input, Some(disk_available)) {
		None => panic!("参数错误：input 设置值过大或其他错误 \r\nopts: {:?}", opts),
		Some(v) => v,
	};

	let output_byte: u64 = match string_to_number_format_u64(&opts.output, Some(disk_available)) {
		None => panic!("参数错误：output 设置值过大或其他错误 \r\nopts: {:?}", opts),
		Some(v) => v,
	};

	let output_byte = if output_byte < input_byte { input_byte } else { output_byte };

	if output_byte >= disk_available {
		panic!("参数错误： output 设置值大于磁盘所剩余值 \r\nopts: {:?}", opts)
	}

	let mut curr_file_path = curr_path.clone();
	curr_file_path.push(PathBuf::from(&opts.filename));
	if !curr_file_path.is_file() {
		File::create_new(&curr_file_path)?;
	}

	let output = vec![0xAB; output_byte as usize];

	loop {
		let mut file =
			OpenOptions::new().write(true).read(true).truncate(true).open(&curr_file_path)?;

		let output_random = random(output_byte);
		let input_random = if output_random < input_byte {
			random(output_random as u64)
		} else {
			random(input_byte)
		};

		let output_random_bytes = &output[0..(random(output_byte) as usize)];

		#[allow(unused_unsafe)]
		unsafe {
			let _ = file.write_all(output_random_bytes);
			let _ = file.flush(); // 写的文件都 flush 到磁盘

			let _ = file.seek(SeekFrom::Start(output_random - input_random)); // 设置 读取的开始位置
			let mut strs = String::new();
			let _ = file.read_to_string(&mut strs);
			println!("读取的文件长度: {}", &strs.len());

			// let _ = file.seek(SeekFrom::Start(0));
			// let mut read: Vec<u8> = vec![0xAB; input_random as usize];
			// let _ = black_box(file.read(read.as_mut_slice())); // 重新读取数据，从 0 位置开始读取

			let _ = file.flush();
			let _ = file.try_clone(); // 关闭文件
		};
		println!(
			"写入数据: {} => {}\r\n读取数据: {} => {}",
			output_random,
			number_format_to_string(output_random),
			input_random,
			number_format_to_string(input_random)
		);
		sleep(Duration::from_secs(refresh as u64));
	}
}

// todo 到时候改成多线程
fn write_bytes(file: &mut File, size: u64, byte_buff: &[u8]) {
	if size >= byte_buff.len() as u64 {
		let count = size / byte_buff.len() as u64;
		let mo = (size % count) as usize;
		for _ in 0..=count {
			let _ = file.write(byte_buff);
		}
		if mo != 0 {
			let _ = file.write(vec![0u8; mo].as_slice());
		}
	} else {
		let _ = file.write_all(vec![0u8; size as usize].as_slice());
	}
}

pub fn process_file(opts: &FileOpts) -> Result<()> {
	let curr_path: PathBuf = PathBuf::from(opts.dirname.as_str()).canonicalize().unwrap();

	if !curr_path.is_dir() {
		panic!("路径不存在: {}", opts.dirname);
	}

	let disks = Disks::new_with_refreshed_list();
	println!("{:?}", disks);
	// 获取 指定路径磁盘的信息
	let curr_disk = disks
		.into_iter()
		.filter(|disk| curr_path.starts_with(disk.mount_point().canonicalize().unwrap())) //
		.max_by(|x, y| x.mount_point().cmp(y.mount_point())) // 取得最大的那个挂载点
		.unwrap();

	let disk_total = curr_disk.total_space();
	let disk_available = curr_disk.available_space();

	println!(
		"磁盘挂载点为: {:?},总值为: {}, 可使用值为: {}",
		curr_disk.mount_point(),
		number_format_to_string(disk_total),
		number_format_to_string(disk_available)
	);

	let byte_buff: Vec<u8> = vec![0u8; number_format_conversion(10, 'm', true) as usize];
	let byte_buff_u8 = byte_buff.as_slice();

	let now = SystemTime::now();
	let total_bytes: u64 = match string_to_number_format_u64(&opts.num, Some(disk_available)) {
		None => panic!("参数错误：num 设置值过大或其他错误 \r\nopts: {:?}", opts),
		Some(v) => v,
	};
	if total_bytes >= disk_available {
		panic!("参数错误：设置值大于磁盘所剩余值 \r\nopts: {:?}", opts)
	}
	if opts.filecount > 1 {
		let count = total_bytes / opts.filecount as u64;
		let mo = total_bytes % count;
		for i in 0..opts.filecount {
			let mut curr_file_path = curr_path.clone();
			curr_file_path.push(PathBuf::from(format!("{}.{}", &opts.filename, i)));
			let mut file = File::create_new(&curr_file_path)?;
			write_bytes(&mut file, count, byte_buff_u8);
			let _ = file.try_clone();
		}
		if mo != 0 {
			let mut curr_file_path = curr_path.clone();
			curr_file_path.push(PathBuf::from(&opts.filename));
			let mut file = File::create_new(&curr_file_path)?;
			write_bytes(&mut file, mo, byte_buff_u8);
			let _ = file.try_clone();
		}
	} else {
		let mut curr_file_path = curr_path.clone();
		curr_file_path.push(PathBuf::from(&opts.filename));
		if curr_file_path.exists() {
			panic!("文件路径: {:?} 下，已经存在文件: {}!", curr_file_path, &opts.filename);
		}
		let mut file = File::create_new(&curr_file_path)?;
		write_bytes(&mut file, total_bytes, byte_buff_u8);
		let _ = file.try_clone();
	}

	println!("新建文件耗时: {:?}", SystemTime::now().duration_since(now));
	Ok(())
}

// #[test]
// fn test_disk() {
//     let opts: IoOpts = IoOpts {
//         dirname: r"D:\test\create_file".to_string(),
//         new: true,
//         num: "10k".to_string(),
//         input: "20m".to_owned(),
//         output: "20m".to_string(),
//         filename: "test_io.txt".to_string(),
//         filecount: 3,
//     };
//     let _ = process(&opts, 1);
// }

// #[test]
// fn test_full_name() {
//     // 指定路径
//     let path = PathBuf::from(".");

//     // 获取指定路径的绝对完整路径
//     let absolute_full_path = path.canonicalize().expect("Failed to get absolute full path");

//     // 打印绝对完整路径
//     println!("Absolute full path: {:?}", absolute_full_path.as_os_str());
// }

// #[test]
// fn test_path_max() {
//     let path1 = PathBuf::from("/");
//     let path2 = PathBuf::from("/root");
//     println!("{:?} {:?}", path1.cmp(&path2), path2.cmp(&path1));
// }
