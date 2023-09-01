use std::fmt::{Display, Formatter};

pub mod nie_validator;

pub enum NIE{
    Local{serial: String},
    Foreign{serial: String}
}

impl Display for NIE{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NIE::Local { serial } => {write!(f, "{serial}")}
            NIE::Foreign { serial } => {write!(f, "{serial}")}
        }
    }
}