use std::io::{self, Write};

mod tax;

fn get_tax_message(price: u32) -> String {
    let tax = tax::calculate_tax(price);

    format!(
        "本体価格{}円に対する消費税は{}円、税込価格は{}円です。",
        price,
        tax,
        price + tax
    )
}

fn main() {
    print!("本体価格を入力してください: ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    let message = match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<u32>() {
            Ok(n) => get_tax_message(n),
            Err(_) => format!(
                "入力可能な金額は 0 <= n <= {} です。",
                u32::max_value()
            ),
        },
        Err(_) => "内容の読み取りに失敗しました".to_string(),
    };

    println!("{}", message);
}
