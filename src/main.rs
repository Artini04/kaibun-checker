use console::style;
use kakasi::convert;
use text_io::read;
use wana_kana::ConvertJapanese;

fn main() {
    println!(
        "{} Text longer than 15 characters will be truncate.",
        style("[NOTE]").yellow()
    );
    print!("{}", style("Enter text: ").bold());
    let line: String = read!("{}\n");
    kaibun_checker(line.as_str());
}

fn kaibun_checker(line: &str) -> bool {
    let line_string = convert(line).hiragana.to_hiragana();
    let line_rev = line_string.chars().rev().collect::<String>();
    let line_len: usize = 15;

    match line_string == line_rev {
        true => {
            println!(
                "{} {} === {}",
                style("[OK]").green(),
                truncate(line_string.as_str(), line_len),
                truncate(line_rev.as_str(), line_len)
            );
            return true;
        }
        false => {
            println!(
                "{} {} =/= {}",
                style("[NO]").red(),
                truncate(line_string.as_str(), line_len),
                truncate(line_rev.as_str(), line_len)
            );
            return false;
        }
    }
}

fn truncate(s: &str, max_char: usize) -> &str {
    match s.char_indices().nth(max_char) {
        None => s,
        Some((idx, _)) => &s[..idx],
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
    )); // Phrases with spaces will return false
}
