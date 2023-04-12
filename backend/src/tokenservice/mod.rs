use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn sign_token() -> String{
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", "someone");
    
    let token_str = claims.sign_with_key(&key).unwrap();
    
    token_str
    // assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
}

pub fn verify_token(token_str: &str){
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    assert_eq!(claims["sub"], "someone")

}