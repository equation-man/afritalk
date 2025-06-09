//! DEFINES CRYPTOGRAPHIC FUNCTIONS THAT ENCODE AND DECODES THE MESSAGE.

/// Encrypting the message to be sent.
/// This method leverages a specified non-singular matrix A
/// for the process of encoding the message.
pub fn encode() {
    println!("This is the function to encode the message");
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
