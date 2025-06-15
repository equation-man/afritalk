//! DEFINES CRYPTOGRAPHIC FUNCTIONS THAT ENCODE AND DECODES THE MESSAGE.
use utils::utils::DEFAULT_CORRESP; 
use utils::utils::{
    encoding_test_mat,
    gen_correspondence,
    message_numerals,
    message_vector,
    matrix_mul,
};

/// Encrypting the message to be sent.
/// This method leverages a specified non-singular matrix A
/// for the process of encoding the message.
pub fn encode() {
    let corresp_mat = gen_correspondence(&DEFAULT_CORRESP);
    println!("This is the message correspondent {:?}", corresp_mat);
}

pub fn decode() {
    println!("This is the function to decode the message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        encode();
    }

    #[test]
    fn test_decode() {
        decode();
    }
}
