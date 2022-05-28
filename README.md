# fhe-oszeca
Fully Homomorphic Encryption OS Projects

# Usage
Just run the following:
```bash
cargo build --release
cargo run --release
```
For faster re-runs, the program stores the _bootstrapping key_ in "keys/default-key.json" and reloads it using the still-not-parametrized parameters (getopts).

# ToDo
- Investigate C++ code directly, like this [one](https://github.com/vernamlab/cuFHE)

# Project
- Evaluate advantages of using sourcehut as our software development platform;
- Evaluate advantages of setting up a self-hosted environment (raspi4 model B);

# Theory
- Gentry's published paper on FHE, [here](https://www.cs.cmu.edu/~odonnell/hits09/gentry-homomorphic-encryption.pdf)
- The so-called first FHE, proposed in Gentry's PhD, [here](https://crypto.stanford.edu/craig/craig-thesis.pdf);
- A good introduction on FHE, [here](https://eprint.iacr.org/2015/1192.pdf);
- [Concrete](https://github.com/zama-ai/concrete)'s library [whitepaper](https://eprint.iacr.org/2018/421.pdf);
  - Running a function over a float, [here](https://docs.zama.ai/concrete/lib/user/bootstrapped-operations/applying-a-function-to-a-ciphertext.html);
- Partially Homomorphic Encryption: El Gamal and Paillier;
- Relations of FHE: Functional Encryption and Program Obfuscation;
- FHE's limitation for multi-user being tackled down by [multi-key FHE](https://www.iacr.org/archive/tcc2016b/99850157/99850157.pdf);
  
# References
- [Here](https://github.com/jonaschn/awesome-he) is a curated list of FHE libs, software and resources.
