use std::hash::pedersen_hash;

fn main(secret: Field, public_hash: pub Field) {
    let computed_hash = pedersen_hash([secret]);
    assert(computed_hash == public_hash);
}

#[test]
fn test_main() {
    main(
        0x04e94fe643fe9000c83dd91f0be27855aa2cd791a3dfc1e05775749e89f4693e,
        2,
    );
}
