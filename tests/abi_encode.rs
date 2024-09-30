use syslib::{abi::{Encodable}, types::ints::U256};
#[test]
fn intx() {
    let int = U256::from_usize(123);

    let r = int.encode();
}
