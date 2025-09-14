
#[derive(Debug)]
pub enum PoolError {
    InvalidPoolData,
    DivisionByZero,
    PriceCalculationFailed,
}

impl std::fmt::Display for PoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for PoolError {}
