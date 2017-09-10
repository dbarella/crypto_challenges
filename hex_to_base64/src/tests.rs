use ::base64;
use ::hex;
use ::convert;

#[test]
fn convert_hex_to_base64() {
    let hex = hex::Hex::new();
    let base64 = base64::Base64::new();

    assert_eq!(convert(hex), base64);
}
