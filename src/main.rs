use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::short_weierstrass::{
        curves::bls12_381::curve::BLS12381Curve, point::ShortWeierstrassProjectivePoint,
    },
    elliptic_curve::traits::IsEllipticCurve,
    traits::{AsBytes, ByteConversion},
    unsigned_integer::element::U256,
};

/// This function computes the public key from a given secret key
/// on the BLS12_381 curve. It uses elliptic curve multiplication
/// of the secret key with the generator point to derive the public key.
pub fn derive_public_key_from_secret(secret_key_input: U256) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let curve_generator = BLS12381Curve::generator();
    let derived_public_key = curve_generator.operate_with_self(secret_key_input);
    derived_public_key
}

fn main() {
    let input_secret_key = U256::from_hex_unchecked("6C616D6264617370");
    let derived_public_key = derive_public_key_from_secret(input_secret_key);
    let public_key_bytes = derived_public_key.as_bytes();
    let public_key_as_u256 =
        U256::from_bytes_be(&public_key_bytes).expect("Failed to convert public key to U256");

    println!("Derived Public Key: {:?}", public_key_as_u256.to_hex());
}
