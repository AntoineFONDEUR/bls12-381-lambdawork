use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::compression;

fn main(){
    let sk: u64 = 0x6C616D6264617370;

    let g1 = BLS12381Curve::generator();
    let pk = g1.operate_with_self(sk);
    let pk_compressed = compression::compress_g1_point(&pk);
}
