/// Main blueprint for mayas project,  pulls in locking blueprint for staking and locking tokens as well
/// as blueprint that manages all users/stats with individual NFTs.

use scrypto::prelude::*;
// use crate::mayaslock::*;  // do we need a seperate component to create locked vaults?
use crate::mayasuser::*;


blueprint! {
    struct Mayas {
        // Define what resources and data will be managed by Mayas components
        admin_badge_add: ResourceAddress,
        access_badge_add: ResourceAddress,

        mayas_token: ResourceAddress,
        mlp_token: ResourceAddress,

        mayas_staking_vault: Vault,
        mlp_staking_vault: Vault,
        hashrate_liquidity_vault: Vault,
    }

    impl Mayas {
        // Implement the functions and methods which will manage those resources and data
        
        // This is a function, and can be called directly on the blueprint once deployed
        pub fn instantiate_mayas(staking_lockup_time: u128) -> (ComponentAddress, Bucket) {
            let admin_badge: Bucket = ResourceBuilder::new_fungible()
                // .mintable(rule!(require(admin_badge_add.resource_address())), LOCKED)
                // .burnable(rule!(require(admin_badge_add.resource_address())), LOCKED)
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Admin")
                .initial_supply(1);

            let access_badge: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)    
                .metadata("name", "Access Badge")
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .initial_supply(1);

            let mayas_token: Bucket = ResourceBuilder::new_fungible()  // cap supply at 21million
                .divisibility(DIVISIBILITY_NONE)    
                .metadata("name", "Access Badge")
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            let mlp_token: Bucket = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)    
                .metadata("name", "Access Badge")
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .no_initial_supply();

            // Instantiate a Hello component, populating its vault with our supply of 1000 HelloToken
            let mut mayas_component = Self {
                
                staking_lockup_time: u64,
                admin_badge_add,
                access_badge_add,
                mayas_token,
                mlp_token,
                access_vault: Vault::with_bucket(access_badge),
                mayas_staking_vault: Vault::new(mayas_token.resource_address()),
                mlp_staking_vault: Vault::new(mlp_token.resource_address()),
                hashrate_liquidity_vault: Vault::new(), // accept bitcoin, wbtc here?
            }
            .instantiate();
            let mayas_component_add: ComponentAddress = mayas_component.globalize();

            (mayas_component_add, admin_badge)
        }

        /// Method for investors holding Mayas tokens that want the right to earn 20% of all BTC 
        /// hashrate.
        pub fn stake_mayas(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {  
            // assert bucket = mayas token resource address
            // return staking receipt or no?
            // require proof of user NFT in order to track total stake/time of stake per user
        }

        /// Method for investors holding Mayas tokens that want the right to earn 20% of all BTC 
        /// hashrate.
        pub fn unstake_mayas(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {  
            // assert bucket = mayas token resource address
            // return staking receipt or no?
            // require proof of user NFT in order to track total stake/time of stake per user
        }

        /// Method for Miners, and Mayas-BTC LP holders
        pub fn stake_mlp(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {  
            // assert bucket = MLP resource address (this is just an LP token from a dex right?)
            // return staking receipt or no?
            // require proof of user NFT in order to track total stake/time of stake per user
        }

        /// Method for Miners, and Mayas-BTC LP holders
        pub fn unstake_mlp(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {  
            // assert bucket = MLP resource address (this is just an LP token from a dex right?)
            // return staking receipt or no?
            // require proof of user NFT in order to track total stake/time of stake per user
        }

        /// Method for Miners to collateralize hashrate they are providing that has been verified
        /// by an oracle.
        pub fn collateralize_hashrate(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {
            // miner deposits funds and "locks" them to collateralize hashrate
        }

        // Method for investors of hashrate, half paired with hashrate for liquidity
        pub fn invest_in_hashrate(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {
            // user can send in BTC - 
            //half will go to hashrate buy wallet, half goes to collateralize that hashrate
        }

        pub fn claim_mining_collateral(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {
            // Miners can claim collateral (if not sold) after completing the 1 year block period
            // Burn associated MLP tokens
        }


        pub fn report_incorrect_hashrate(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {

        }

        pub fn settle_hashrate_report_bounty(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {
            // take locked liquidity from those misreporting and give to those reporting the error
        }

        pub fn get_oracle_data_verify_hashrate(&mut self, mayas: Bucket, user_nft: Proof) -> Bucket {
            
        }


        










    }
}