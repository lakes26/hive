
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub enum TournamentStatus {
    #[default]
    NotStarted,
    InProgress,
    Finished,
}

impl fmt::Display for TournamentStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let game_status = match self {
            Self::NotStarted => String::from("NotStarted"),
            Self::InProgress => String::from("InProgress"),
            Self::Finished => String::from("Finished"),
        };
        write!(f, "{game_status}")
    }
}

impl FromStr for TournamentStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NotStarted" => Ok(TournamentStatus::NotStarted),
            "InProgress" => Ok(TournamentStatus::InProgress),
            "Finished" => Ok(TournamentStatus::Finished),
            _ => Err(anyhow::anyhow!("Invalid TournamentStatus string")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_game_status() {
        for gc in [
            TournamentStatus::NotStarted,
            TournamentStatus::InProgress,
            TournamentStatus::Finished,
        ]
        .iter()
        {
            assert_eq!(Ok(gc.clone()), TournamentStatus::from_str(&format!("{gc}")));
        }
    }
}
