/// 本体価格から消費税を計算する。
///
/// 端数処理は大多数の事業者が採用している切り捨て方式を用いている。
pub fn calculate_tax(price: u32) -> u32 {
    ((price as f32) * 0.08).ceil() as u32
}

#[cfg(test)]
mod tests_calculate_tax {
    use super::calculate_tax;

    #[test]
    fn it_works() {
        assert_eq!(calculate_tax(100), 8);
    }
}
