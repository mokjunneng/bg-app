#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Currency {
    MYR,
    SGD,
}

#[derive(Debug, Clone, Copy)]
pub struct Price {
    pub currency: Currency,
    // TODO: Work with integer values for now
    pub value: u32,
}
impl Price {
    pub fn new(currency: Currency, value: u32) -> Self {
        Self { currency, value }
    }
}
impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}
impl PartialEq for Price {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
