use crate::types::uintx::{IntX, LE};

use super::{Encodable, EncodingError};

impl<'a, const N: usize> Encodable for IntX<N, LE> {
    const ENCODED_SLOTS_FIT_BYTES: usize = 32;
    const ENCODED_SLOTS_FIT: usize = 1;
    fn encode(&self) -> Result<(), EncodingError>
        where [(); Self::ENCODED_SLOTS_FIT_BYTES]: 
    {
        loop {}
    }

    fn encoded_size(&self) -> usize  { 32 }
}



