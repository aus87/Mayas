// use sbor::decode;
use scrypto::prelude::*;
// use std::iter::FromIterator;

// User NFT is an NFT that represents users for this protocol. This NFT contains all the records of the user
// interacting with this protocol.
#[derive(NonFungibleData)]
pub struct MayasUser {
    #[scrypto(mutable)]
    pub handle: String,  // Name, handle, discord, telegram etc
    #[scrypto(mutable)]
    pub mayas_staking_record: KeyValueStore<Decimal, Decimal>, // <value staked, time staked>
    #[scrypto(mutable)]
    pub mlp_staking_record: KeyValueStore<Decimal, Decimal>, // <value staked, time staked>
    #[scrypto(mutable)]
    pub total_current_mlp_stake: Decimal,
    #[scrypto(mutable)]
    pub total_current_mayas_stake: Decimal,
}

// This enum describes the different ranks of repuatation for each user
#[derive(TypeId, Encode, Decode, Describe, Debug, PartialEq)]
pub enum HashrateType { 
    // different stages of hashrate
    Uncollateralized, 
    Collateralized, 
}

blueprint! {
    // Everything here deals with the SBT data management.
    struct MayasUser {
        
        // Vault that holds the authorization badge
        access_badge_vault: Vault, 
        // Collects User SBT Address
        sbt_address: ResourceAddress, 
        // This is the user record registry. It is meant to allow people to query the users that belongs to this protocol.
        user_record: HashMap<NonFungibleId, User>, 
        // Keeps a record of wallet addresses to ensure that maps 1 SBT to 1 Account.
        account_record: Vec<ComponentAddress>, 
        // NFT ID, rep_balance from sbt
    }

    // Instantiates the XRDaoUser component. This is instantiated through the XRDao component. 
    impl MayasUser {
        
        pub fn instantiate_mayasuser(access_badge_address: ResourceAddress) -> ComponentAddress{
            let access_rules: AccessRules = AccessRules::new()
                .method("new_user", rule!(require(access_badge_address)))
                // .method("", rule!(require(access_badge_address)))
                .default(rule!(allow_all));

            // Badge that will be stored in the component's vault to provide authorization to update the User NFT.
            let access_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("user", "XRDao Protocol Badge")
                .initial_supply(1);

            // NFT description for user identification. 
            let sbt_address = ResourceBuilder::new_non_fungible()
                .metadata("user", "XRDao User")
                .mintable(rule!(require(access_badge.resource_address())), LOCKED)
                .burnable(rule!(require(access_badge.resource_address())), LOCKED)
                .restrict_withdraw(rule!(deny_all), LOCKED)
                .updateable_non_fungible_data(rule!(require(access_badge.resource_address())), LOCKED)
                .no_initial_supply();
            
            let component = Self {
                access_badge_vault: Vault::with_bucket(access_badge),
                sbt_address: sbt_address,
                user_record: HashMap::new(), 
                account_record: Vec::new(), // collected to make sure only 1 SBT per 1 account address
            }

            .instantiate();
            component.add_access_check(access_rules);
            (component.globalize(), access_badge)
        }
    
        // Creates a Soul Bount Token (SBT) for a new user to the XRDao Protocol
        // **Check 1:** Checks for 1 SBT per 1 account address.
        // Takes handle, and users account address
        // Returns bucket with user's SBT
        pub fn new_user(&mut self, account_address: ComponentAddress, handle: String) -> Bucket {

            // Checks whether the account address has already registered an SBT
            assert_ne!(self.account_record.contains(&account_address), true, "SBT already created for this account.");
            
            let new_user_sbt: Bucket = self.access_badge_vault.authorize(|| {  // Mint NFT to give to users as identification
                let resource_manager: &ResourceManager = borrow_resource_manager!(self.sbt_address);
                resource_manager.mint_non_fungible(
                    &NonFungibleId::random(),
                    // The starting User data with handle from user input, with no balances, empty voting record, and lowest rank
                    MayasUser { 
                        handle: handle,
                        mayas_staking_record: KeyValueStore::new(),
                        mlp_staking_record: KeyValueStore::new(),
                        total_current_mlp_stake: dec!(0),
                        total_current_mayas_stake: dec!(0),

                    },
                    new_user_sbt
                )
            });

            // creates user reward vault
            let user_reward_vault: Vault = Vault::new(RADIX_TOKEN);
            // finds user SBT ID
            let user_id: NonFungibleId = new_user_sbt.non_fungible::<User>().id();
            // Stores SBT ID, Vault for user
            self.user_reward_vaults.insert(user_id, user_reward_vault);
            // Stores SBT data
            let sbt_data: MayasUser = new_user_sbt.non_fungible().data();
            // updates records held within component
            self.user_record.insert(user_id, sbt_data);
            self.account_record.push(account_address);

            new_user_sbt // Returns NFT to user
        }
    
        // Method call to authorize and increase user xrdao balance on SBT
        pub fn add_mlp_stake(&mut self, user_id: &NonFungibleId, amount: Decimal, time: Decimal) {
            // let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            // *sbt_data.xrdao_balance += amount; // Increases the xrdao balance value stored on SBT
            // self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }

        // Method call to authorize and decrease user xrdao balance on SBT
        pub fn dec_mlp_stake(&mut self, user_id: &NonFungibleId, amount: Decimal) {
            // let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            // *sbt_data.xrdao_balance -= amount; // Decreases the xrdao balance value stored on SBT
            // self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.
        }     

        // Method call to authorize and increase user rep balance on SBT
        pub fn add_mayas_stake(&mut self, user_id: &NonFungibleId, amount: Decimal, time: Decimal) { 
            // let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            // *sbt_data.rep_balance += amount; // Increases the rep balance value stored on SBT
            // self.authorize_update(&sbt_id, sbt_data); // Authorizes the update to SBT.

            // // updates rep_leaderboard hashmap stored in xrdaouser component
            // // If the rep leaderboard doesnt contain a key = user's NFT ID
            // if self.rep_leaderboard.contains_key(&sbt_id) = false { 
            //     // Then insert into leaderboard hashmap this users NFT ID, amount of rep deposited
            //     self.rep_leaderboard.insert(&sbt_id, amount); 
            // // otherwise, update the existing value for this key by adding amount of rep deposited to it
            // } else {  
            //     self.rep_leaderboard.get_mut(&sbt_id).unwrap() += amount;
            // }
            // update_rank(user_id);
        }

        // Method call to authorize and decrease user rep balance on SBT
        pub fn dec_mayas_stake(&mut self, user_id: &NonFungibleId, amount: Decimal, time: Decimal) {
            // let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            // *sbt_data.rep_balance -= amount; // Decreases the rep balance value stored on SBT
            // self.authorize_update(&user_id, user_sbt); // Authorizes the update to SBT.
            
            // //remove key and value from hashmap for sbt_id if there is no rep balance
            // self.rep_leaderboard.get_mut(&sbt_id).unwrap() -= amount; 
            //     if user_sbt_data.rep_balance == 0 {
            //         self.rep_leaderboard.remove(&sbt_id);
            //     }
    
            // update_rank(user_id);
        } 

        // This method is used to allow users retrieval of their SBT data.
        pub fn get_sbt_info(&self, user_id: NonFungibleId) {
            let (sbt_id, mut sbt_data) = self.check_and_retrieve_user(user_id);
            let handle = sbt_data.handle;
            let xrdao_balance = sbt_data.xrdao_balance;
           
            info!("[User SBT]: Handle: {:?}", handle);
            info!("[User SBT]: Mayas Staking Record: {:?}", mayas_staking_record);
            info!("[User SBT]: mlp_staking_record: {:?}", mlp_staking_record);
            info!("[User SBT]: total_current_mlp_stake: {:?}", total_current_mlp_stake);
            info!("[User SBT]: total_current_mayas_stake: {:?}",  total_current_mayas_stake);
        }

        // Asserts that the Proof is for a XRDao user SBT
        // Returns user SBT ID, SBT data
        fn check_and_retrieve_user(&self, sbt: Proof) -> (NonFungibleId, User) {  
            assert_eq!(sbt.resource_address(), self.sbt_address, "Unsupported user SBT");
            let sbt_id = sbt.non_fungible::<User>().id();
            self.retrieve_user_from_id(sbt_id);
            (sbt_id, sbt_data)
        }

        /// Takes SBT ID and returns the ID and the data from the SBT
        fn retrieve_user_from_id(&self, sbt_id: NonFungibleId) -> (NonFungibleId, User) {
            let sbt_manager = borrow_resource_manager!(self.sbt_address);
            let sbt_data = sbt_manager.get_non_fungible_data(&sbt_id);
            (sbt_id, sbt_data)
        }

        // This is a helper function to borrow the resource manager
        // Takes `user_id` (&NonFungibleId) and returns User struct
        fn call_resource_mananger(&self, user_id: &NonFungibleId) -> User {
            let resource_manager = borrow_resource_manager!(self.sbt_address);
            let sbt: User = resource_manager.get_non_fungible_data(&user_id);
            sbt
        }

        // Authorizes data update for the User SBT.
        // Arguments: `user_sbt` (User), `user_id` (&NonFungibleId)
        // Returns: nothing
        fn authorize_update(&mut self, user_id: &NonFungibleId, user_sbt: User) {
            let resource_manager = borrow_resource_manager!(self.sbt_address);
            self.sbt_badge_vault.authorize(|| resource_manager.update_non_fungible_data(&user_id, user_sbt));
        }

    }
    
}