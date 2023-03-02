use scrypto::prelude::*;

#[blueprint]
mod ScryptoBunnies {
    struct Bunnies {
        bunnies_token_vault: Vault,
        // bunnies_nft_vault: Vault
    }

    impl Bunnies {
        pub fn instantiate_bunnies() -> ComponentAddress {
            // Bunnies Token
            let bunnites_token_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "Pelletz")
                .metadata("symbol", "PLTZ")
                .mint_initial_supply(/* Eventually a choice will be made!! */);

            // Bunnies NFT
            // start here...

            Self {
                bunnies_token_vault: Vault::with_bucket(bunnites_token_bucket)
            }
                .instantiate()
                .globalize()
        }
    }
}