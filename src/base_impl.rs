use super::base::*;

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.get_stateid() == other.get_stateid()
    }
}

impl Eq for Player {}

