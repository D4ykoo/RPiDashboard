use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub fn sign_token() -> String{
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret-nftoppings-19823adsf123").unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("user", "dario-ahljef9827364l,kajuh1");
    
    let token_str = claims.sign_with_key(&key).unwrap();
    
    token_str
    // assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
}

pub fn verify_token(token_str: &str) -> bool{
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret-nftoppings-19823adsf123").unwrap();

    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    if claims["sub"] == "dario-ahljef9827364l,kajuh1"{return true};
    return false;
    // assert_eq!(claims["sub"], "someone")

}