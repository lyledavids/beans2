#![cfg_attr(not(feature = "export-abi"), no_main)]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;

use alloy_primitives::Address;
use alloy_sol_types::sol;
// use alloy_primitives::Uint;
use stylus_sdk::{
    alloy_primitives::U256,
    prelude::*, 
    call::transfer_eth, 
    //storage::{SimpleStorageType, Erase, StorageAddress, StorageString}
};

sol! {
    event devLog(string name, string description, string githubURL, address indexed wallet);
    event RewardLog(address indexed donor, address indexed developer, uint256 indexed amount);
}

sol_storage! {
    #[entrypoint]
    pub struct Beans {        
        // mapping(string => string) names;
        // mapping(string => string) descriptions;
        // mapping(string => string) githubURLs;
        // mapping(address => uint) wallets;
        // BeansDeveloper[] beans;
        address addr;
        string name;
        uint age;

    }

    //User[] public users;


    // pub struct BeansDeveloper {
    //     string name;
    //     string description;
    //     string githubURL;
    //     address wallet;
    // }
}

// #[solidity_storage]
// pub struct BeansStruct {
//     name: StorageString,
//     description: StorageString,
//     githubURL: StorageString,
//     wallet: StorageAddress
// }



#[external]
impl Beans {
    
    pub fn send(to: Address, amount: U256) -> Result<(), Vec<u8>> {
        
        transfer_eth(to, amount)?; 
        Ok(())
    }

    // pub fn send_multiple(transfers: &[(Address, U256)]) -> Result<(), Vec<u8>> {
    //     for (to, amount) in transfers {
    //         transfer_eth(*to, *amount)?;
    //     }
    //     Ok(())
    // }
}