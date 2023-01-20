use kakasi::convert;
use text_io::read;
use wana_kana::ConvertJapanese;

fn main() {
    print!("Enter line: ");
    let line: String = read!("{}\n");
    kaibun_checker(line.as_str());
}

fn kaibun_checker(line: &str) -> bool {
    let line_string = convert(line).hiragana.to_hiragana();
    let line_rev = line_string.chars().rev().collect::<String>();

    match line_string == line_rev {
        true => {
            println!("[/] {} : {}", line_string, line_rev);
            return true;
        }
        false => {
            println!("[X] {} : {}", line_string, line_rev);
            return false;
        }
    }
}

#[test]
fn test_kaibun_checker() {
    // Kaibun
    assert!(kaibun_checker("たけやくやけた"));
    assert!(kaibun_checker("私負けましたわ"));
    assert!(kaibun_checker("なるとを取るな"));
    assert!(kaibun_checker("シナモンパンもレモンパンも無し"));

    // Non Kaibun
    assert!(!kaibun_checker("いせかい"));
    assert!(!kaibun_checker(
        "長き世の 遠の眠りの 皆目覚め 波乗り船の 音の良きかな"
    )); // Phrases with half-width spaces will return false
}
