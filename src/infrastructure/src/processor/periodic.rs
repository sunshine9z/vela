use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct PeriodicJob{
    pub name: String,
    pub class: String,
}