use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnBattle {
    pub success: bool,
    pub hand: Vec<String>,
    pub small_runes: Vec<String>,
    pub enemy_small_runes: Vec<String>,
    pub player_hp: u64,
    pub enemy_hp: u64,
    pub enemy_hand_size: usize,
    pub mana: u64,
    pub enemy_mana: u64,
    pub hexa_runes: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TakeAction {
    pub play_card: usize,
    pub character_id : i64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BattleErrors {
    ChosenCardNotInHand(usize),
    CardCostsTooMuch {
        chosen: usize,
        mana_available: u64,
        mana_needed: u64,
    },
}
impl std::error::Error for BattleErrors {}
impl Display for BattleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            BattleErrors::ChosenCardNotInHand(x) => x.to_string(),
            BattleErrors::CardCostsTooMuch {
                chosen,
                mana_available,
                mana_needed,
            } => format!(
                "chosen : {} , mana_available: {}, mana_neede : {}",
                chosen, mana_available, mana_needed
            ),
        };
        write!(f, "{}", v)
    }
}
#[cfg(feature = "server")]
impl From<BattleErrors> for rlua::Error {
    fn from(error: BattleErrors) -> Self {
        Self::external(error)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TurnResponse {
    NextTurn(ReturnBattle),
    Error(BattleErrors),
    Done,
}
