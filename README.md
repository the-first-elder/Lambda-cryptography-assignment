# BLS12-381 Public Key Derivation Example

This repository contains a simple Rust program that demonstrates how to derive a public key from a secret key using the BLS12-381 elliptic curve. The program uses the `lambdaworks_math` library, which provides efficient implementations of cryptographic primitives, including elliptic curve operations and unsigned integer arithmetic.

## Features

- **BLS12-381 Curve**: Utilizes the BLS12-381 elliptic curve for key derivation.
- **Secret Key Input**: Accepts a secret key as input, which is used to derive the public key.
- **Public Key Derivation**: Implements elliptic curve multiplication to derive the public key from the secret key.
- **Output Formatting**: Converts the derived public key to a hexadecimal string for easy readability.

## Getting Started

### Prerequisites

- Rust: Ensure you have Rust installed on your system. You can download it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- `lambdaworks_math`: This example uses the `lambdaworks_math` library for cryptographic operations. You can add it to your `Cargo.toml` file as shown below:

```toml
[dependencies]
lambdaworks_math = "0.1.0" # Use the latest version available
```

### Running the Program

1. Clone this repository or copy the code into a new Rust project.
```sh
https://github.com/the-first-elder/Lambda-cryptography-assignment.git  
```
2. Run the program using Cargo:

```bash
cargo run
```

3. The program will output the derived public key in hexadecimal format.

## Example Output

```plaintext
Derived Public Key: EFC2D10AD531CEBF2B8C7B4325BC93ED91E6477D260304C1F9ECC7BA0E6F5711
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.