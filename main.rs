use tfhe::{ConfigBuilder, generate_keys, set_server_key, FheUint8};
use tfhe::prelude::*;
use std::u64;
use std::time::{Duration, Instant};

fn square_and_multiply(base:u64, exponent :u64, modulus: u64) -> u64 {

    if exponent == 0 {
        return base;
    }

    let mut z = 1;
    let mut b = base % modulus;
    let mut e = exponent;

    for _i in 0..exponent-1 {
        if e %2 == 1 {
            z = (z*b) % modulus;
        }

        e=e >> 1;
        b = (b*b)% modulus;
    }

    return z;
}

// p = 13
// q = 17
// n = 17 * 13 = 221 
// lambda(p*q) = lcm(p-1, q-1) = 48
// e = 5
// d = 29
// de = 1 mod 48

fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let start = Instant::now();

    // Client-side
    let (client_key, server_key) = generate_keys(config);
    
    let key_generation_time: Duration = start.elapsed();


    let clear_e = 29u8; // esponente segreto aka chiave
    let m = 42u8; // il messaggio da firmare, è in CHIARO!
    let n = 221u8; // il modulo, fa parte della chiave pubblica.

    // cifro la chiave
    let e = FheUint8::encrypt(clear_e, &client_key);
    

    let mod_pow = |value: u64| {
        square_and_multiply(m.into(), value, n.into()) as u64
    };

    //Server-side
    set_server_key(server_key);
    
    // map applica TFHE programmable bootstrapping.
    // Call it magic!
    let result = e.map(mod_pow);
    let sqmul_time: Duration = start.elapsed();


    //Client-side
    let decrypted_result: u8 = result.decrypt(&client_key);

    print!("key generation: {:?}, square multiply/programmable bootstrapping{:?}", key_generation_time, sqmul_time-key_generation_time);
    
    //42^29 mod 221 = 9 mod 221
    assert_eq!(decrypted_result, 9);

}

