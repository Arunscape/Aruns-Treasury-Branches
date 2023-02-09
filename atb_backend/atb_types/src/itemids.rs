
pub enum ItemId {
    Diamond,
}

impl TryFrom<&str> for ItemId {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "diamond" => Ok(ItemId::Diamond),
            _ => Err(()),
        }
    }
}