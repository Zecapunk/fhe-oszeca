use concrete::*;
use std::path::Path;
use std::fs::create_dir_all;

fn main() -> Result<(), CryptoAPIError> {
    // encoders
    let encoder_input = Encoder::new(-10., 10., 6, 1)?;
    let encoder_output = Encoder::new(0., 100., 6, 0)?;

    // secret keys
    let sk_rlwe = RLWESecretKey::new(&RLWE128_1024_1);
    let sk_in = LWESecretKey::new(&LWE128_630);
    let sk_out = sk_rlwe.to_lwe_secret_key();
    let bsk: LWEBSK;
    let default_key_path = "keys/default-key.json";
    
    if Path::new(&default_key_path).exists() {
        println!("Reloading default key at '{}'", &default_key_path);
        bsk = LWEBSK::load(&default_key_path);
    } else {
        let base_log: usize = 5;
        let level: usize = 3;
        bsk = LWEBSK::new(&sk_in, &sk_rlwe, base_log, level);
        match create_dir_all("keys") {
            Ok(_) => bsk.save(&default_key_path),
            Err(why) => println!("Não foi possível salvar a chave... {}", why),
        };
    }

    // message
    let message: f64 = -5.;

    // encode and encrypt
    let c1 = LWE::encode_encrypt(&sk_in, message, &encoder_input)?;

    // bootstrap
    let c2 = c1.bootstrap_with_function(&bsk, |x| x * x, &encoder_output)?;

    // decrypt
    let output = c2.decrypt_decode(&sk_out)?;

    println!("before bootstrap: {}, after bootstrap: {}", message, output);

    Ok(())
}

// fn generate_bootstrap_key(sk_in: LWESecretKey, sk_rlwe: RLWESecretKey) -> LWEBSK {
//     // bootstrapping key and its settings
//     let base_log: usize = 5;
//     let level: usize = 3;
//     LWEBSK::new(&sk_in, &sk_rlwe, base_log, level)
// }
