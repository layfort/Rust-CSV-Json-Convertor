mod utils;

use std::io::Write;

use utils::card_struct::{AttackCard, BaseCard, Card, ColorCard, MoveCard};
use utils::constants::{CARD_DATA_PATH, OUTPUT_PATH};

fn main() {
    let mut csv_reader = csv::Reader::from_path(CARD_DATA_PATH).expect("Read csv file error");
    let mut base_card: Vec<BaseCard> = Vec::new();

    for record in csv_reader.deserialize() {
        let record: BaseCard = record.expect("Deserialize error");
        base_card.push(record);
    }

    let mut attack_cards: Vec<AttackCard> = Vec::new();
    let mut color_cards: Vec<ColorCard> = Vec::new();
    let mut move_cards: Vec<MoveCard> = Vec::new();

    for card in base_card {
        if card.get_type() == "Attack".to_string() {
            let attack_card = AttackCard::new(card);
            attack_cards.push(attack_card);
        } else if card.get_type() == "Color".to_string() {
            let color_card = ColorCard::new(card);
            color_cards.push(color_card);
        } else {
            let move_card = MoveCard::new(card);
            move_cards.push(move_card);
        }
    }

    let mut attack_card_data = serde_json::to_string_pretty(&attack_cards).unwrap();
    let mut color_card_data = serde_json::to_string_pretty(&color_cards).unwrap();
    let mut move_card_data = serde_json::to_string_pretty(&move_cards).unwrap();

    let attack_card_data = attack_card_data.drain(0..attack_card_data.len() - 2).as_str().to_string();
    let color_card_data = color_card_data.drain(1..color_card_data.len() - 2).as_str().to_string();
    let move_card_data = move_card_data.drain(1..move_card_data.len()).as_str().to_string();

    let total_data = format!(
        r#"{}, {}, {}"#,
        attack_card_data, color_card_data, move_card_data
    );

    println!("{}", total_data);

    let mut writer = std::io::BufWriter::new(std::fs::File::create(OUTPUT_PATH).unwrap());
    let _ = writer.write_all(total_data.as_bytes()).unwrap();
}
