use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Copy, Clone,Deserialize,Serialize)]
pub enum Faction {
    Rebel,
    Beladimir,
    Lerastir,
    Menoriad,
}