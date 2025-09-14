pub mod error;
pub mod meteora;
pub mod orca;
pub mod raydium;

pub mod structs {
    use crate::pools_struct::error::PoolError;

    #[derive(Debug, Clone, Copy)]
    pub enum DexType {
        Meteora,
        HumidiFi,
        SolFiV2,
        PancakeSwap,
        Lifinity,
        Orca,
        Raydium,
    }

    pub trait PriceFetcher {
        /// Get the current SOL/USDC price
        fn get_price(&self) -> Result<f64, PoolError>;
        /// Get the DEX name
        fn get_dex_name(&self) -> &'static str;
    }
}
