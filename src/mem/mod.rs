#[allow(clippy::module_inception)]
pub mod mem {
    use crate::{
        cmd::MemoryOpts,
        tools::{number_format_to_string, random, string_to_number_format_u64},
    };
    use std::alloc::{alloc, dealloc, Layout};
    use std::hint::black_box;
    use std::mem::align_of;
    use std::ptr;
    use std::{thread::sleep, time::Duration};
    use sysinfo::System;

    pub fn process(opts: &MemoryOpts, refresh: u8) {
        let mut sys: System = System::new_all();
        // 获取内存总量
        let total_memory: u64 = sys.total_memory();
        let used_memory: u64 = sys.used_memory();

        let target_memory: u64 = match string_to_number_format_u64(&opts.num, Some(total_memory)) {
            None => panic!("参数错误：设置值过大或其他错误 \r\nopts: {:?}", opts),
            Some(v) => v,
        };

        if target_memory >= total_memory {
            panic!("参数错误，目标值：{} 大于实际数据: {}", target_memory, total_memory);
        }

        println!(
            "总值：{} \r\n当前值：{} \r\n目标值：{}",
            number_format_to_string(total_memory),
            number_format_to_string(used_memory),
            number_format_to_string(target_memory)
        );

        loop {
            sys.refresh_memory();
            // 获取内存使用信息
            let used_memory: u64 = sys.used_memory();
            if target_memory > used_memory {
                let target_memory_random: u64 = random(target_memory - used_memory);
                println!("================ \r\n当前值：{} \r\n插入值：{}\r\n目标值：{}\r\n================",number_format_to_string(used_memory),number_format_to_string(target_memory_random),number_format_to_string(target_memory));
                handler(target_memory_random as usize, refresh);
            }
        }
    }

    fn handler(size: usize, refresh: u8) {
        // 创建一个 u8 布局 ， size 长度的 layout 对象
        let layout = Layout::from_size_align(size, align_of::<u8>()).unwrap();

        // 分配内存, black_box 函数标记 内存对象是有意义的，避免被 编译器优化掉
        let ptr = black_box(unsafe { alloc(layout) });

        if !ptr.is_null() {
            println!("内存：{} 分配成功！", size);
            // 给内存写满值
            unsafe {
                ptr::write_bytes(ptr, 0xAB, size);
            }
            println!("内存：{} 写值成功！", size);
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
