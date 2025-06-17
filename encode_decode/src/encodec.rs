//! DEFINES CRYPTOGRAPHIC FUNCTIONS THAT ENCODE AND DECODES THE MESSAGE.
use utils::utils::DEFAULT_CORRESP; 
use utils::utils::{
    encoding_test_mat,
    gen_correspondence,
    message_numerals,
    message_vector,
    matrix_mul,
    transpose,
};

/// MESSAGE ENCRYPTION.
/// This method leverages a specified non-singular matrix A
/// for the process of encoding the message.
/// Encoding the message involves multiplying the message matrix
/// and the specified non-singuar matrix A to produce the encoded message.
pub fn encode(msg: String, encoding_mat: Vec<usize>, correspondence: &[char]) {
    let corresp_mat = gen_correspondence(correspondence);
    // Converting message to its numerical equivalence using correspondence mat.
    let mut msg_numeric = message_numerals(msg, &corresp_mat);
    // Obtaining the desired message vector.
    let mut msg_vec = message_vector(&mut msg_numeric);
    transpose(&mut msg_vec, 8);
    // Encoding the message.
    //let mut mat_mul = matrix_mul(encoding_mat, msg_mat);
    // Converting the message matrix into text.
}

pub fn decode() {
    println!("This is the function to decode the message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let msg = "Today is my favourite day. Tommorow will be another great day.".to_string();
        //encode(msg, &DEFAULT_CORRESP);
    }

    #[test]
    fn test_decode() {
        decode();
    }
}
