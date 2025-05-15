# Registration Contract

A simple but secure registration contract built with ink! that allows users to register with a signature verification mechanism.

## Overview

This contract provides a secure way to register users on the blockchain. Each user can register only once, and registration requires a valid signature to prevent unauthorized registrations.

## Features

- Secure registration with signature verification
- Prevention of double registration
- Simple and efficient storage using ink!'s Mapping
- Comprehensive test coverage
- End-to-end testing support

## Prerequisites

- Rust and Cargo
- ink! toolchain
- Substrate development environment (for end-to-end tests)

## Building

To build the contract:

```bash
cargo build
```

For optimized build:

```bash
cargo build --release
```

## Testing

Run the unit tests:

```bash
cargo test
```

Run the end-to-end tests:

```bash
cargo test --features e2e-tests
```

## Contract Interface

### Messages

#### `register_with_signature`

```rust
pub fn register_with_signature(
    &mut self,
    message: Vec<u8>,
    signature: Vec<u8>,
) -> Result<(), Error>
```

Registers the caller if they provide a valid signature for the message.

- `message`: The message that was signed
- `signature`: The signature of the message
- Returns: `Ok(())` if registration is successful, `Err(Error)` otherwise

#### `is_registered`

```rust
pub fn is_registered(&self, account: AccountId) -> bool
```

Checks if an account is registered.

- `account`: The account to check
- Returns: `true` if the account is registered, `false` otherwise

### Errors

The contract can return the following errors:

- `AlreadyRegistered`: The account is already registered
- `InvalidSignature`: The provided signature is invalid

## Usage Example

```rust
// Create a message to sign
let message = b"Register me".to_vec();

// Sign the message (implementation depends on your environment)
let signature = sign_message(&message, account_id);

// Register with the signature
contract.register_with_signature(message, signature)?;

// Check registration status
let is_registered = contract.is_registered(account_id);
```

## Development

### Project Structure

```
my_contract/
├── Cargo.toml
├── lib.rs
└── README.md
```

### Dependencies

- `ink = "5.0.0"`: The ink! smart contract framework
- `scale = { package = "parity-scale-codec", version = "3.0.0" }`: For encoding/decoding
- `scale-info = "2.1.1"`: For type information

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Security

This contract is provided as-is with no guarantees. Please ensure you understand the code and its implications before using it in production.

## Future Improvements

- Implement proper signature verification for production use
- Add registration time tracking
- Add ability to unregister
- Add registration fees
- Add additional user information storage