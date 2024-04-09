use rand::Rng;

/**
 * 参数单位格式转换
 */
pub fn string_to_number_conversion(val: &str, unit: char) -> Option<u64> {
    let (num_str, unit_str) = val.split_at(val.len() - 1);

    let val_unit: char = unit_str.chars().last().unwrap().to_ascii_lowercase();
    let val_number: u64 = num_str.parse::<u64>().unwrap();

    if val_unit == unit.to_ascii_lowercase() {
        Some(val_number)
    } else {
        // if
        match number_format_conversion(
            number_format_conversion(val_number, val_unit, true),
            unit,
            false,
        ) {
            0 => None,
            v => Some(v),
        }
    }
}

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
