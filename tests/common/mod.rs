use bloock_babyjubjub_rs::{verify, PrivateKey};
use num_bigint::{BigInt, Sign};

pub fn circomlib_testvector() {
    // test private key
    let sk = PrivateKey::import(
        hex::decode("0001020304050607080900010203040506070809000102030405060708090001").unwrap(),
    )
    .unwrap();

    assert_eq!(
        sk.scalar_key().to_string(),
        "6466070937662820620902051049739362987537906109895538826186780010858059362905"
    );

    // test public key
    let pk = sk.public();
    assert_eq!(
        pk.x.to_string(),
        "Fr(0x1d5ac1f31407018b7d413a4f52c8f74463b30e6ac2238220ad8b254de4eaa3a2)"
    );
    assert_eq!(
        pk.y.to_string(),
        "Fr(0x1e1de8a908826c3f9ac2e0ceee929ecd0caf3b99b3ef24523aaab796a6f733c4)"
    );

    // test signature & verification
    let msg = BigInt::from_bytes_le(Sign::Plus, &hex::decode("00010203040506070809").unwrap());
    let sig = sk.sign(msg.clone()).unwrap();
    assert_eq!(
        sig.r_b8.x.to_string(),
        "Fr(0x192b4e51adf302c8139d356d0e08e2404b5ace440ef41fc78f5c4f2428df0765)"
    );
    assert_eq!(
        sig.r_b8.y.to_string(),
        "Fr(0x2202bebcf57b820863e0acc88970b6ca7d987a0d513c2ddeb42e3f5d31b4eddf)"
    );
    assert_eq!(
        sig.s.to_string(),
        "1672775540645840396591609181675628451599263765380031905495115170613215233181"
    );
    let v = verify(pk, sig, msg);
    assert_eq!(v, true);
}
