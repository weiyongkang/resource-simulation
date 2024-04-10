use rand::Rng;

/**
 * 参数格式转换
 *
 */
pub fn string_to_number_format_u64(val: &str, total: Option<u64>) -> Option<u64> {
    let (num_str, unit_str) = val.split_at(val.len() - 1);

    let val_unit: char = unit_str.chars().last().unwrap().to_ascii_lowercase();
    let val_number: u64 = num_str.parse::<u64>().unwrap();

    match &val_unit {
        '%' => total.map(|v| ((val_number as f64 / 100f64) * v as f64).ceil() as u64),
        c => match number_format_conversion(val_number, *c, true) {
            0 => None,
            vv => Some(vv),
        },
    }
}

/**
 * io 单位换算
 */
pub fn number_format_conversion(num: u64, unit: char, ad: bool) -> u64 {
    if ad {
        match unit.to_ascii_lowercase() {
            'b' => num,
            'k' => num * 1024,
            'm' => num * 1024 * 1024,
            'g' => num * 1024 * 1024 * 1024,
            _ => 0,
        }
    } else {
        match unit.to_ascii_lowercase() {
            'b' => num,
            'k' => num / 1024,
            'm' => num / 1024 / 1024,
            'g' => num / 1024 / 1024 / 1024,
            _ => 0,
        }
    }
}

/**
 * 通过 值获得io 单位
 */
pub fn number_format_unit(num: u64) -> char {
    match num {
        x if x < 1024 => 'b',
        x if x < 1024 * 1024 => 'k',
        x if x < 1024 * 1024 * 1024 => 'm',
        _ => 'g',
    }
}

/**
 * 把 字节 类型 格式化转换成更便于读取的格式
 */
pub fn number_format_to_string(num: u64) -> String {
    match num {
        x if x < 1024 => format!("{}{}", num, 'b'),
        x if x < 1024 * 1024 => format!("{}{}", num / 1024, 'k'),
        x if x < 1024 * 1024 * 1024 => format!("{}{}", num / 1024 / 1024, 'm'),
        _ => format!("{}{}", num / 1024 / 1024 / 1024, 'g'),
    }
}

/**
 * 获得一个指定范围内的随机数
 */
pub fn random(num: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let to_first = (num as f64 * 0.85f64).ceil() as u64;
    // 生成一个在指定范围内的随机整数
    rng.gen_range(to_first..num) as u64
}

// #[test]
// fn test_regex(){
//     let test_val:Vec<&str> = vec!["10k","89G","9m","dd","ddsda10k","50%"];
//     for test in test_val {
//         match value_parser_format(test) {
//             Ok(v) => println!("{}:{:?}",&v,string_to_number_conversion(&v,'m')),
//             Err(v) => println!("{}",v)
//         }
//     }
// }

// #[test]
// fn test_unit_format() {
//     let num: usize = number_format_conversion(100, 'M', true) as usize;
//     let tim: Duration = Duration::from_secs(600);

//     let mem = vec![0; num];

//     sleep(tim);
//     println!("{}", num)
// }
