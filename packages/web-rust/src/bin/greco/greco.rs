use std::ops::Deref;
use std::sync::Arc;

use fhe::bfv::{Ciphertext, Plaintext, PublicKey};
use fhe_math::rq::{Poly, Representation};
use fhe_math::{rq::Context, zq::Modulus};

use itertools::izip;
use num_bigint::BigInt;
use num_traits::*;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::greco::poly::*;

/// Assign and return all of the centered input validation vectors to the ZKP modulus `p`.
///
/// # Arguments
///
/// * `pk0is` - Centered coefficients of first public key object for each RNS modulus
/// * `pk1is` - Centered coefficients of second public key object for each RNS modulus
/// * `r2is` - Centered coefficients of r2 for each RNS modulus
/// * `r1is` - Centered coefficients of r1 for each RNS modulus
/// * `p2is` - Centered coefficients of p2 for each RNS modulus
/// * `p1is` - Centered coefficients of p1 for each RNS modulus
/// * `ct0is` - Centered coefficients of first ciphertext object for each RNS modulus
/// * `ct1is` - Centered coefficients of second ciphertext object for each RNS modulus
/// * `u` - Centered coefficients of secret polynomial used during encryption (sampled from secret key distribution)
/// * `e0` - Centered coefficients of error polynomial used during encryption (sampled from error distribution)
/// * `e1` - Centered coefficients of error polynomial used during encryption (sampled from error distribution)
/// * `k1` - Centered coefficients of [Q*m] mod t
/// * `p` - ZKP modulus
///
pub fn input_validation_vectors_standard_form(
    pk0is: &[Vec<BigInt>],
    pk1is: &[Vec<BigInt>],
    r2is: &[Vec<BigInt>],
    r1is: &[Vec<BigInt>],
    p2is: &[Vec<BigInt>],
    p1is: &[Vec<BigInt>],
    ct0is: &[Vec<BigInt>],
    ct1is: &[Vec<BigInt>],
    u: &[BigInt],
    e0: &[BigInt],
    e1: &[BigInt],
    k1: &[BigInt],
    p: &BigInt,
) -> (
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<BigInt>,
    Vec<BigInt>,
    Vec<BigInt>,
    Vec<BigInt>,
) {
    (
        reduce_coefficients_2d(pk0is, p),
        reduce_coefficients_2d(pk1is, p),
        reduce_coefficients_2d(r2is, p),
        reduce_coefficients_2d(r1is, p),
        reduce_coefficients_2d(p2is, p),
        reduce_coefficients_2d(p1is, p),
        reduce_coefficients_2d(ct0is, p),
        reduce_coefficients_2d(ct1is, p),
        reduce_coefficients(u, p),
        reduce_coefficients(e0, p),
        reduce_coefficients(e1, p),
        reduce_coefficients(k1, p),
    )
}

/// Create the centered validation vectors necessary for creating an input validation proof according to Greco.
/// For more information, please see https://eprint.iacr.org/2024/594.
///
/// # Arguments
///
/// * `ctx` - Context object from fhe.rs holding information about elements in Rq.
/// * `t` - Plaintext modulus object.
/// * `pt` - Plaintext from fhe.rs.
/// * `u_rns` - Private polynomial used in ciphertext sampled from secret key distribution.
/// * `e0_rns` - Error polynomial used in ciphertext sampled from error distribution.
/// * `e1_rns` - Error polynomioal used in cihpertext sampled from error distribution.
/// * `ct` - Ciphertext from fhe.rs.
/// * `pk` - Public Key from fhe.re.
///
pub fn compute_input_validation_vectors(
    ctx: &Arc<Context>,
    t: &Modulus,
    pt: &Plaintext,
    u_rns: &Poly,
    e0_rns: &Poly,
    e1_rns: &Poly,
    ct: &Ciphertext,
    pk: &PublicKey,
) -> (
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<BigInt>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<Vec<BigInt>>,
    Vec<BigInt>,
    Vec<BigInt>,
    Vec<BigInt>,
    Vec<BigInt>,
) {
    let N: u64 = ctx.degree as u64;

    // Calculate k1 (independent of qi), center and reverse
    let q_mod_t = (ctx.modulus() % t.modulus()).to_u64().unwrap(); // [q]_t
    let mut k1_u64 = pt.value.deref().to_vec(); // m
    t.scalar_mul_vec(&mut k1_u64, q_mod_t); // k1 = [q*m]_t
    let mut k1: Vec<BigInt> = k1_u64.iter().map(|&x| BigInt::from(x)).rev().collect();
    reduce_and_center_coefficients_mut(&mut k1, &BigInt::from(t.modulus()));

    // Extract single vectors of u, e1, and e2 as Vec<BigInt>, center and reverse
    let mut u_rns_copy = u_rns.clone();
    let mut e0_rns_copy = e0_rns.clone();
    let mut e1_rns_copy = e1_rns.clone();
    u_rns_copy.change_representation(Representation::PowerBasis);
    e0_rns_copy.change_representation(Representation::PowerBasis);
    e1_rns_copy.change_representation(Representation::PowerBasis);
    let u: Vec<BigInt> = unsafe {
        ctx.moduli_operators()[0]
            .center_vec_vt(u_rns_copy.coefficients().row(0).as_slice().unwrap())
            .iter()
            .map(|&x| BigInt::from(x))
            .rev()
            .collect()
    };

    let e0: Vec<BigInt> = unsafe {
        ctx.moduli_operators()[0]
            .center_vec_vt(e0_rns_copy.coefficients().row(0).as_slice().unwrap())
            .iter()
            .map(|&x| BigInt::from(x))
            .rev()
            .collect()
    };

    let e1: Vec<BigInt> = unsafe {
        ctx.moduli_operators()[0]
            .center_vec_vt(e1_rns_copy.coefficients().row(0).as_slice().unwrap())
            .iter()
            .map(|&x| BigInt::from(x))
            .rev()
            .collect()
    };

    // Extract and convert ciphertext and plaintext polynomials
    let mut ct0 = ct.c[0].clone();
    let mut ct1 = ct.c[1].clone();
    ct0.change_representation(Representation::PowerBasis);
    ct1.change_representation(Representation::PowerBasis);

    let mut pk0: Poly = pk.c.c[0].clone();
    let mut pk1: Poly = pk.c.c[1].clone();
    pk0.change_representation(Representation::PowerBasis);
    pk1.change_representation(Representation::PowerBasis);

    // Create cyclotomic polynomial x^N + 1
    let mut cyclo = vec![BigInt::from(0u64); (N + 1) as usize];
    cyclo[0] = BigInt::from(1u64); // x^N term
    cyclo[N as usize] = BigInt::from(1u64); // x^0 term

    // Print
    /*
    println!("m = {:?}\n", &m);
    println!("k1 = {:?}\n", &k1);
    println!("u = {:?}\n", &u);
    println!("e0 = {:?}\n", &e0);
    println!("e1 = {:?}\n", &e1);
     */

    // Initialize matrices to store results
    let num_moduli = ctx.moduli().len();
    let mut r2is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut r1is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut k0is: Vec<BigInt> = vec![BigInt::zero(); num_moduli];
    let mut ct0is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut ct0is_hat: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut ct1is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut ct1is_hat: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut pk0is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut pk1is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut p1is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];
    let mut p2is: Vec<Vec<BigInt>> = vec![Vec::new(); num_moduli];

    // Initialize iterators for results calculation
    let moduli_operators = ctx.moduli_operators();
    let ct0_iter = ct0.coefficients();
    let ct1_iter = ct1.coefficients();
    let pk0_iter = pk0.coefficients();
    let pk1_iter = pk1.coefficients();
    let zipped: Vec<_> = izip!(
        moduli_operators,
        ct0_iter.rows(),
        ct1_iter.rows(),
        pk0_iter.rows(),
        pk1_iter.rows()
    )
    .collect();

    // Perform the main computation logic
    let results: Vec<(
        usize,
        Vec<BigInt>,
        Vec<BigInt>,
        BigInt,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
        Vec<BigInt>,
    )> = zipped
        .into_par_iter()
        .enumerate()
        .map(
            |(i, (qi, ct0_coeffs, ct1_coeffs, pk0_coeffs, pk1_coeffs))| {
                // --------------------------------------------------- ct0i ---------------------------------------------------

                // Convert to vectors of bigint, center, and reverse order.
                let mut ct0i: Vec<BigInt> =
                    ct0_coeffs.iter().map(|&x| BigInt::from(x)).rev().collect();
                let mut ct1i: Vec<BigInt> =
                    ct1_coeffs.iter().map(|&x| BigInt::from(x)).rev().collect();
                let mut pk0i: Vec<BigInt> =
                    pk0_coeffs.iter().map(|&x| BigInt::from(x)).rev().collect();
                let mut pk1i: Vec<BigInt> =
                    pk1_coeffs.iter().map(|&x| BigInt::from(x)).rev().collect();

                let qi_bigint = BigInt::from(qi.modulus());

                reduce_and_center_coefficients_mut(&mut ct0i, &qi_bigint);
                reduce_and_center_coefficients_mut(&mut ct1i, &qi_bigint);
                reduce_and_center_coefficients_mut(&mut pk0i, &qi_bigint);
                reduce_and_center_coefficients_mut(&mut pk1i, &qi_bigint);

                // k0qi = -t^{-1} mod qi
                let koqi_u64 = qi.inv(qi.neg(t.modulus())).unwrap();
                let k0qi = BigInt::from(koqi_u64); // Do not need to center this

                // ki = k1 * k0qi
                let ki = poly_scalar_mul(&k1, &k0qi);

                // Calculate ct0i_hat = pk0 * ui + e0i + ki
                let ct0i_hat = {
                    let pk0i_times_u = poly_mul(&pk0i, &u);
                    assert_eq!((pk0i_times_u.len() as u64) - 1, 2 * (N - 1));

                    let e0_plus_ki = poly_add(&e0, &ki);
                    assert_eq!((e0_plus_ki.len() as u64) - 1, N - 1);

                    poly_add(&pk0i_times_u, &e0_plus_ki)
                };
                assert_eq!((ct0i_hat.len() as u64) - 1, 2 * (N - 1));

                // Check whether ct0i_hat mod R_qi (the ring) is equal to ct0i
                let mut ct0i_hat_mod_rqi = ct0i_hat.clone();
                reduce_in_ring(&mut ct0i_hat_mod_rqi, &cyclo, &qi_bigint);
                assert_eq!(&ct0i, &ct0i_hat_mod_rqi);

                // Compute r2i numerator = ct0i - ct0i_hat and reduce/center the polynomial
                let ct0i_minus_ct0i_hat = poly_sub(&ct0i, &ct0i_hat);
                assert_eq!((ct0i_minus_ct0i_hat.len() as u64) - 1, 2 * (N - 1));
                let mut ct0i_minus_ct0i_hat_mod_zqi = ct0i_minus_ct0i_hat.clone();
                reduce_and_center_coefficients_mut(&mut ct0i_minus_ct0i_hat_mod_zqi, &qi_bigint);

                // Compute r2i as the quotient of numerator divided by the cyclotomic polynomial
                // to produce: (ct0i - ct0i_hat) / (x^N + 1) mod Z_qi. Remainder should be empty.
                let (r2i, r2i_rem) = poly_div(&ct0i_minus_ct0i_hat_mod_zqi, &cyclo);
                assert!(r2i_rem.is_empty());
                assert_eq!((r2i.len() as u64) - 1, N - 2); // Order(r2i) = N - 2

                // Assert that (ct0i - ct0i_hat) = (r2i * cyclo) mod Z_qi
                let r2i_times_cyclo = poly_mul(&r2i, &cyclo);
                let mut r2i_times_cyclo_mod_zqi = r2i_times_cyclo.clone();
                reduce_and_center_coefficients_mut(&mut r2i_times_cyclo_mod_zqi, &qi_bigint);
                assert_eq!(&ct0i_minus_ct0i_hat_mod_zqi, &r2i_times_cyclo_mod_zqi);
                assert_eq!((r2i_times_cyclo.len() as u64) - 1, 2 * (N - 1));

                // Calculate r1i = (ct0i - ct0i_hat - r2i * cyclo) / qi mod Z_p. Remainder should be empty.
                let r1i_num = poly_sub(&ct0i_minus_ct0i_hat, &r2i_times_cyclo);
                assert_eq!((r1i_num.len() as u64) - 1, 2 * (N - 1));

                let (r1i, r1i_rem) = poly_div(&r1i_num, &[qi_bigint.clone()]);
                assert!(r1i_rem.is_empty());
                assert_eq!((r1i.len() as u64) - 1, 2 * (N - 1)); // Order(r1i) = 2*(N-1)
                assert_eq!(&r1i_num, &poly_mul(&r1i, &[qi_bigint.clone()]));

                // Assert that ct0i = ct0i_hat + r1i * qi + r2i * cyclo mod Z_p
                let r1i_times_qi = poly_scalar_mul(&r1i, &qi_bigint);
                let mut ct0i_calculated =
                    poly_add(&poly_add(&ct0i_hat, &r1i_times_qi), &r2i_times_cyclo);

                while ct0i_calculated.len() > 0 && ct0i_calculated[0].is_zero() {
                    ct0i_calculated.remove(0);
                }

                assert_eq!(&ct0i, &ct0i_calculated);

                // --------------------------------------------------- ct1i ---------------------------------------------------

                // Calculate ct1i_hat = pk1i * ui + e1i
                let ct1i_hat = {
                    let pk1i_times_u = poly_mul(&pk1i, &u);
                    assert_eq!((pk1i_times_u.len() as u64) - 1, 2 * (N - 1));

                    poly_add(&pk1i_times_u, &e1)
                };
                assert_eq!((ct1i_hat.len() as u64) - 1, 2 * (N - 1));

                // Check whether ct1i_hat mod R_qi (the ring) is equal to ct1i
                let mut ct1i_hat_mod_rqi = ct1i_hat.clone();
                reduce_in_ring(&mut ct1i_hat_mod_rqi, &cyclo, &qi_bigint);
                assert_eq!(&ct1i, &ct1i_hat_mod_rqi);

                // Compute p2i numerator = ct1i - ct1i_hat
                let ct1i_minus_ct1i_hat = poly_sub(&ct1i, &ct1i_hat);
                assert_eq!((ct1i_minus_ct1i_hat.len() as u64) - 1, 2 * (N - 1));
                let mut ct1i_minus_ct1i_hat_mod_zqi = ct1i_minus_ct1i_hat.clone();
                reduce_and_center_coefficients_mut(&mut ct1i_minus_ct1i_hat_mod_zqi, &qi_bigint);

                // Compute p2i as the quotient of numerator divided by the cyclotomic polynomial,
                // and reduce/center the resulting coefficients to produce:
                // (ct1i - ct1i_hat) / (x^N + 1) mod Z_qi. Remainder should be empty.
                let (p2i, p2i_rem) = poly_div(&ct1i_minus_ct1i_hat_mod_zqi, &cyclo.clone());
                assert!(p2i_rem.is_empty());
                assert_eq!((p2i.len() as u64) - 1, N - 2); // Order(p2i) = N - 2

                // Assert that (ct1i - ct1i_hat) = (p2i * cyclo) mod Z_qi
                let p2i_times_cyclo: Vec<BigInt> = poly_mul(&p2i, &cyclo);
                let mut p2i_times_cyclo_mod_zqi = p2i_times_cyclo.clone();
                reduce_and_center_coefficients_mut(&mut p2i_times_cyclo_mod_zqi, &qi_bigint);
                assert_eq!(&ct1i_minus_ct1i_hat_mod_zqi, &p2i_times_cyclo_mod_zqi);
                assert_eq!((p2i_times_cyclo.len() as u64) - 1, 2 * (N - 1));

                // Calculate p1i = (ct1i - ct1i_hat - p2i * cyclo) / qi mod Z_p. Remainder should be empty.
                let p1i_num = poly_sub(&ct1i_minus_ct1i_hat, &p2i_times_cyclo);
                assert_eq!((p1i_num.len() as u64) - 1, 2 * (N - 1));

                let (p1i, p1i_rem) = poly_div(&p1i_num, &[BigInt::from(qi.modulus())]);
                assert!(p1i_rem.is_empty());
                assert_eq!((p1i.len() as u64) - 1, 2 * (N - 1)); // Order(p1i) = 2*(N-1)
                assert_eq!(&p1i_num, &poly_mul(&p1i, &[qi_bigint.clone()]));

                // Assert that ct1i = ct1i_hat + p1i * qi + p2i * cyclo mod Z_p
                let p1i_times_qi = poly_scalar_mul(&p1i, &qi_bigint);
                let mut ct1i_calculated =
                    poly_add(&poly_add(&ct1i_hat, &p1i_times_qi), &p2i_times_cyclo);

                while ct1i_calculated.len() > 0 && ct1i_calculated[0].is_zero() {
                    ct1i_calculated.remove(0);
                }

                assert_eq!(&ct1i, &ct1i_calculated);

                /*
                println!("qi = {:?}\n", &qi_bigint);
                println!("ct0i = {:?}\n", &ct0i);
                println!("k0qi = {:?}\n", &k0qi);
                println!("pk0 = Polynomial({:?})\n", &pk0i);
                println!("pk1 = Polynomial({:?})\n", &pk1i);
                println!("ki = {:?}\n", &ki);
                println!("ct0i_hat_mod_rqi = {:?}\n", &ct0i_hat_mod_rqi);
                */

                (
                    i, r2i, r1i, k0qi, ct0i, ct0i_hat, ct1i, ct1i_hat, pk0i, pk1i, p1i, p2i,
                )
            },
        )
        .collect();

    // println!("Completed creation of polynomials!");

    // Aggregate results into global vectors
    for (i, r2i, r1i, k0i, ct0i, ct0i_hat, ct1i, ct1i_hat, pk0i, pk1i, p1i, p2i) in
        results.into_iter()
    {
        r2is[i] = r2i;
        r1is[i] = r1i;
        k0is[i] = k0i;
        ct0is[i] = ct0i;
        ct0is_hat[i] = ct0i_hat;
        ct1is[i] = ct1i;
        ct1is_hat[i] = ct1i_hat;
        pk0is[i] = pk0i;
        pk1is[i] = pk1i;
        p1is[i] = p1i;
        p2is[i] = p2i;
    }

    (
        r2is, r1is, k0is, ct0is, ct1is, pk0is, pk1is, p1is, p2is, u, e0, e1, k1,
    )
}
