#[allow(clippy::module_inception)]
pub mod mem {
    use crate::{
        cmd::MemoryOpts,
        tools::{number_format_conversion, random},
    };
    use std::alloc::{alloc, dealloc, Layout};
    use std::mem::align_of;
    use std::ptr;
    use std::{thread::sleep, time::Duration};
    use sysinfo::System;

    pub fn process(opts: &MemoryOpts, refresh: u8) {
        let mut sys: System = System::new_all();
        let (num_str, unit_str) = opts.num.split_at(opts.num.len() - 1);

        let val_unit: char = unit_str.chars().last().unwrap().to_ascii_lowercase();
        let val_number = num_str.parse::<u64>().unwrap();

        // 获取内存总量
        let total_memory: u64 = sys.total_memory();
        let used_memory: u64 = sys.used_memory();

        let target_memory: u64 = match val_unit {
            '%' => ((val_number as f64 / 100f64) * total_memory as f64).ceil() as u64,
            v => number_format_conversion(val_number, v, true),
        };

        if target_memory > total_memory {
            panic!("参数错误，目标值：{} 大于实际数据: {}", target_memory, total_memory);
        }

        println!(
            "最大值：{} \r\n当前值：{} \r\n目标值：{}",
            total_memory, used_memory, target_memory
        );

        loop {
            sys.refresh_memory();
            // 获取内存使用信息
            let used_memory: u64 = sys.used_memory();
            if target_memory > used_memory {
                let target_memory_random: u64 = random(target_memory - used_memory);
                println!("================ \r\n当前值：{} \r\n插入值：{}\r\n目标值：{}\r\n================",used_memory,target_memory_random,target_memory);
                handler(target_memory_random as usize, refresh);
            }
        }
    }

    fn handler(size: usize, refresh: u8) {
        // 创建一个 u8 布局 ， size 长度的 layout 对象
        let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();

        // 分配内存
        let ptr = unsafe { alloc(layout) };

        if !ptr.is_null() {
            println!("内存：{} 分配成功！", size);
            // 给内存写满值
            unsafe {
                ptr::write_bytes(ptr, 0xAB, size);
            }

            sleep(Duration::from_secs(refresh as u64));

            // 释放内存
            unsafe {
                dealloc(ptr, layout);
            }
            println!("内存：{} 释放成功！", size);
        } else {
            println!("内存：{} 分配失败！", size);
        }
    }
}

// #[test]
// fn test_men() {
//     use crate::cmd::MemoryOpts;
//     use crate::mem::mem::process;
//     let opts = MemoryOpts { num: "20g".to_string() };

//     process(&opts, 10);
// }
