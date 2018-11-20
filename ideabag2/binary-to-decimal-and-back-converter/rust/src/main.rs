use std::io::{self, Write};

fn get_radix(mode: &str) -> Result<u32, String> {
    println!("{}する進数を選んでください", mode);

    let selections = vec![2, 8, 10, 16];
    
    for i in 0..selections.len() {
        println!("{} ... {}進数", i + 1, selections[i]);
    }

    print!("選択肢: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= selections.len() => {
                let radix = selections[n - 1];

                println!("{} = {}進数\n--- --- ---", mode, radix);

                Ok(radix)
            },
            _ => Err("選択肢にない文字が入力されました".to_string())
        },
        Err(_) => Err("入力内容のパースに失敗しました".to_string())
    }
}

fn main() {
    let input_radix = get_radix("入力");

    if input_radix.is_err() {
        println!("中止します");
        return;
    }

    let output_radix = get_radix("出力");

    if output_radix.is_err() {
        println!("中止します");
        return;
    }

    print!("数値を入力してください: ");
    io::stdout().flush().unwrap();

    let mut num = String::new();

    io::stdin().read_line(&mut num).unwrap();

    let n = u32::from_str_radix(num.trim(), input_radix.unwrap()).expect("パースに失敗");

    match output_radix.unwrap() {
        2 => println!("結果 = {:b}", n),
        8 => println!("結果 = {:o}", n),
        10 => println!("結果 = {}", n),
        16 => println!("結果 = {:x}", n),
        _ => ()
    };
}
