
#![no_std]

imports!();

#[elrond_wasm_derive::callable(StdReferenceInterface)]
pub trait StdReferenceInterface {
    #[callback(set_price_callback)]
    fn get_reference_data(
        &self,
        base_symbol: Vec<u8>,
        quote_symbol: Vec<u8>,
        #[callback_arg] cb_base_symbol: Vec<u8>,
    ) -> SCResult<MultiResult3<BigUint, u64, u64>>;
}

#[elrond_wasm_derive::contract(LSPDBImpl)]
pub trait LSPDB {

    #[init]
    fn init(&self) {
    }

    #[view(getPrice)]
    #[storage_get("price")]
    fn get_price(&self, base_symbol: Vec<u8>) -> BigUint;

    #[view(getStdReference)]
    #[storage_get("std_reference_address")]
    fn get_std_reference_address(&self) -> Address;

    #[storage_set("std_reference_address")]
    fn set_std_reference_address(&self, address: &Address);

    #[storage_set("std_reference_address")]
    fn set_price(&self, base_symbol: &[u8], price: &BigUint);

    #[endpoint(setStdReference)]
    fn set_std_reference(&self, address: &Address) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can set std_reference_address");

        self.set_std_reference_address(address);

        Ok(())
    }

    #[endpoint(savePrice)]
    fn save_price(&self, base_symbol: Vec<u8>) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can set price");

        let std_ref = contract_proxy!(self, &self.get_std_reference_address(), StdReferenceInterface);
        std_ref.get_reference_data(base_symbol.clone(), "USD".as_bytes(), base_symbol.clone());

        Ok(())
    }

	#[callback]
	fn set_price_callback(
		&self,
		result: AsyncCallResult<MultiResult3<BigUint, u64, u64>>,
		#[callback_arg] cb_base_symbol: Vec<u8>,
	) {
		match result {
			AsyncCallResult::Ok(cb_price) => {
                let (rate, _, _) = cb_price.into_tuple();
				self.set_price(&cb_base_symbol, &rate);
			},
			AsyncCallResult::Err(_) => {
				// do nothing
			},
		}
	}
}
