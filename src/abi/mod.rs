pub mod overlay;

pub type EncodingResult = Result<(), EncodingError>;

pub struct EncodingError { }

pub trait Encodable: Sized {
    const ENCODED_SLOTS_FIT: usize;
    const ENCODED_SLOTS_FIT_BYTES: usize;

    fn encoded_size(&self) -> usize ;

    fn encode(&self) -> Result<(), EncodingError>
        where [(); Self::ENCODED_SLOTS_FIT_BYTES ]: ;
}
