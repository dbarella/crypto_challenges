use super::base64;
use super::hex;

#[test]
fn convert_hex_to_base64() {
    let hex_input = 
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f6973\
        6f6e6f7573206d757368726f6f6d";

    let expected_base64 =
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

    assert_eq!(base64.from(hex.from(hex_input)), expected_base64);
}
