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

/// MESSAGE ENCODER.
/// This method leverages a specified non-singular matrix A
/// for the process of encoding the message.
/// Encoding the message involves multiplying the message matrix
/// and the specified non-singuar matrix A to produce the encoded message.
pub fn encode(msg: String, encoding_mat: Vec<usize>, correspondence: &[char]) -> Vec<usize> {
    let corresp_mat = gen_correspondence(correspondence);
    // Converting message to its numerical equivalence using correspondence mat.
    let mut msg_numeric = message_numerals(msg, &corresp_mat);
    // Obtaining the desired message vector.
    let mut msg_vec = message_vector(&mut msg_numeric);
    transpose(&mut msg_vec, 8);
    // Encoding the message.
    let mut mat_mul = matrix_mul(encoding_mat, msg_mat);
    // Encoded message matrix.
    mat_mul
}

/// MESSAGE DECODER.
/// Decoding the message involves multiplying the message matrix
/// by the inverse of the non-singular matrix specified during encoding of the message.
/// The final matrix is then decoded from the correspondence char lists to obtain the final string
pub fn decode(encoded_msg_matrix: Vec<usize>, decoding_mat: Vec<usize>, correspondence: &[char]) -> String {
    let corresp_mat = gen_correspondence(correspondence);
    let mut mat_mul = matrix_mul(decoding_mat, encoded_msg_matrix);
    message_mat_to_chars(mat_mul, corresp_mat);
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
