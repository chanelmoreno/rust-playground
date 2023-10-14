// poly morphism

use ethers::types::Address;
use std::str::FromStr;

trait EthAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EthAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Ethereum address string"),
        }
    }
}

impl EthAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_eth_data<T: EthAddress>(address: T) -> Address {
    let converted_addres: Address = address.convert_address().unwrap();
    converted_addres
}

#[cfg(test)]

mod test {
    use super::*;
    // use std::str::FromStr;
    // use ethers::types::Address;

    // use crate::m4_polymorphism::get_eth_data;

    #[test]

    fn tests_poly() {
        let addr: Address =
            Address::from_str("0x71C7656EC7ab88b098defB751B7401B5f6d8976F").unwrap();

        let new_addr: Address = get_eth_data(addr);
        assert_eq!(
            new_addr,
            Address::from_str("0x71C7656EC7ab88b098defB751B7401B5f6d8976F").unwrap()
        );
    }
}
