use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct SampleRecord {
    City: String,
    Region: String,
    Country: String,
    Population: u32,
}

pub trait Card {
    fn get_type(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct BaseCard {
    CardId: u32,
    CardType: String,
    CardName: String,
    CardDesc: String,
    CardCost: u32,
    CardTag: String,
    Rarity: u32,
    CardPool: String,
    TargetCount: u32,
    AttackCount: u32,
    MainEffect: String,
    TriggerCondition: String,
    CostChangeCondition: String,
    AdditionalEffectCondition: String,
    AdditionalEffect: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct AttackCard {
    CardId: u32,
    CardType: String,
    CardName: String,
    CardDesc: String,
    CardCost: u32,
    CardTag: String,
    Rarity: u32,
    CardPool: String,
    TriggerCondition: String,
    CostChangeCondition: String,
    AdditionalEffectCondition: String,
    AdditionalEffect: String,
    TargetCount: u32,
    AttackCount: u32,
    Damage: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ColorCard {
    CardId: u32,
    CardType: String,
    CardName: String,
    CardDesc: String,
    CardCost: u32,
    CardTag: String,
    Rarity: u32,
    CardPool: String,
    TriggerCondition: String,
    CostChangeCondition: String,
    AdditionalEffectCondition: String,
    AdditionalEffect: String,
    ColorTarget: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MoveCard {
    CardId: u32,
    CardType: String,
    CardName: String,
    CardDesc: String,
    CardCost: u32,
    CardTag: String,
    Rarity: u32,
    CardPool: String,
    TriggerCondition: String,
    CostChangeCondition: String,
    AdditionalEffectCondition: String,
    AdditionalEffect: String,
    MoveDirection: String,
}

impl Card for BaseCard {
    fn get_type(&self) -> String {
        self.CardType.clone()
    }
}

impl AttackCard {
    pub fn new(card: BaseCard) -> AttackCard {
        AttackCard {
            CardId: card.CardId,
            CardType: card.CardType,
            CardName: card.CardName,
            CardDesc: card.CardDesc,
            CardCost: card.CardCost,
            CardTag: card.CardTag,
            Rarity: card.Rarity,
            CardPool: card.CardPool,
            TriggerCondition: card.TriggerCondition,
            CostChangeCondition: card.CostChangeCondition,
            AdditionalEffectCondition: card.AdditionalEffectCondition,
            AdditionalEffect: card.AdditionalEffect,
            TargetCount: card.TargetCount,
            AttackCount: card.AttackCount,
            Damage: card
                .MainEffect
                .trim()
                .parse::<u32>()
                .expect(format!("Damage is not a number: {}", card.MainEffect).as_str()),
        }
    }
}

impl ColorCard {
    pub fn new(card: BaseCard) -> ColorCard {
        ColorCard {
            CardId: card.CardId,
            CardType: card.CardType,
            CardName: card.CardName,
            CardDesc: card.CardDesc,
            CardCost: card.CardCost,
            CardTag: card.CardTag,
            Rarity: card.Rarity,
            CardPool: card.CardPool,
            TriggerCondition: card.TriggerCondition,
            CostChangeCondition: card.CostChangeCondition,
            AdditionalEffectCondition: card.AdditionalEffectCondition,
            AdditionalEffect: card.AdditionalEffect,
            ColorTarget: card.MainEffect,
        }
    }
}

impl MoveCard {
    pub fn new(card: BaseCard) -> MoveCard {
        MoveCard {
            CardId: card.CardId,
            CardType: card.CardType,
            CardName: card.CardName,
            CardDesc: card.CardDesc,
            CardCost: card.CardCost,
            CardTag: card.CardTag,
            Rarity: card.Rarity,
            CardPool: card.CardPool,
            TriggerCondition: card.TriggerCondition,
            CostChangeCondition: card.CostChangeCondition,
            AdditionalEffectCondition: card.AdditionalEffectCondition,
            AdditionalEffect: card.AdditionalEffect,
            MoveDirection: card.MainEffect,
        }
    }
}
