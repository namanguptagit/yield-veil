use arcis::*;
// Now this matches the entry in your Cargo.toml
use arcium_macros::*;

#[encrypted]
mod private_dex {
    use arcis::*;

    #[derive(Copy, Clone)]
    pub struct Order {
        pub is_buy: bool,
        pub price: u64,
        pub size: u64,
        pub owner: [u8; 32],
    }
}