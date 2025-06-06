use base64::{engine::general_purpose::STANDARD, Engine};
    
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}

pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, base64::DecodeError> {
    STANDARD.decode(input)
}
