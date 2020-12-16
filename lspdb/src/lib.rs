
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
    ) -> SCResult<(BigUint, u64, u64)>;
}


#[elrond_wasm_derive::contract(LSPDBImpl)]
pub trait LSPDB {

    #[init]
    fn init(&self) {
    }

    #[storage_get("price")]
    fn get_price_storage(&self, base_symbol: Vec<u8>) -> BigUint;

    #[storage_get("std_reference_address")]
    fn get_std_reference_storage(&self) -> Address;

    #[storage_set("price")]
    fn set_price(&self, base_symbol: &[u8], price: &BigUint);

    #[storage_set("std_reference_address")]
    fn set_std_reference_address(&self, address: &Address);

    #[view(getPrice)]
    fn get_price(&self, base_symbol: Vec<u8>) -> SCResult<BigUint> {
        let price = self.get_price_storage(base_symbol);
        require!(&price > &0, "PRICE_NOT_SET");
        Ok(price)
    }

    #[view(getStdReference)]
    fn get_std_reference(&self) -> SCResult<Address> {
        Ok(self.get_std_reference_storage())
    }

    #[endpoint(setStdReference)]
    fn set_std_reference(&self, address: &Address) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can set std_reference_address");
        self.set_std_reference_address(address);
        Ok(())
    }

    #[endpoint(savePrice)]
    fn save_price(&self, base_symbol: Vec<u8>) -> SCResult<()> {
        require!(self.get_caller() == self.get_owner_address(), "only owner can save price");

        let std_reference:Address = sc_try!(self.get_std_reference());
        let std_ref = contract_proxy!(self, &std_reference, StdReferenceInterface);
        std_ref.get_reference_data(base_symbol.clone(), b"USD".to_vec(), base_symbol.clone());

        self.set_price(b"AAA", &BigUint::from(1234u64));

        Ok(())
    }

	#[callback]
	fn set_price_callback(
		&self,
		result: AsyncCallResult<(BigUint, u64, u64)>,
		#[callback_arg] cb_base_symbol: Vec<u8>,
	) {
		match result {
			AsyncCallResult::Ok(cb_price) => {
                let (rate, _, _) = cb_price;
				self.set_price(&cb_base_symbol, &rate);
			},
			AsyncCallResult::Err(_) => {
				self.set_price(b"BBB", &BigUint::from(999u64));
			},
		}
	}
}

