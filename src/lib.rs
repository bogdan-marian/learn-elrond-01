#![no_std]

imports!();

#[elrond_wasm_derive::contract(CrowdfundingImpl)]
pub trait Crowdfunding {

    #[storage_set("owner")]
    fn set_owner(&self, address: &Address);

    #[view]
    #[storage_get("owner")]
    fn get_owner(&self) -> Address;

    #[init]
    fn init(&self) {
        let my_address: Address = self.get_caller();
        self.set_owner(&my_address);
    }
}
