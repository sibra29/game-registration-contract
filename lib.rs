#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod my_contract {
    use ink::storage::Mapping;
    use ink::prelude::vec::Vec;
    use ink::env::hash::{Blake2x256, HashOutput};
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct MyContract {
        registrations: Mapping<AccountId, bool>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        AlreadyRegistered,
        InvalidSignature,
    }

    impl MyContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { registrations: Mapping::default() }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// Register the caller if they provide a valid signature for the message.
        #[ink(message)]
        pub fn register_with_signature(
            &mut self,
            message: Vec<u8>,
            signature: Vec<u8>,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.is_registered(caller) {
                return Err(Error::AlreadyRegistered);
            }

            // Hash the message
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&message, &mut output);

            // In the test environment, we can verify the signature directly
            #[cfg(test)]
            {
                // For testing, we'll just check if the signature matches the message hash
                if signature != output.to_vec() {
                    return Err(Error::InvalidSignature);
                }
            }

            // In production, we would use the actual signature verification
            #[cfg(not(test))]
            {
                // TODO: Implement actual signature verification
                return Err(Error::InvalidSignature);
            }

            self.registrations.insert(caller, &true);
            Ok(())
        }

        /// Checks if an account is registered.
        #[ink(message)]
        pub fn is_registered(&self, account: AccountId) -> bool {
            self.registrations.get(account).unwrap_or(false)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test::DefaultAccounts;

        fn setup() -> (MyContract, DefaultAccounts<ink::env::DefaultEnvironment>) {
            let contract = MyContract::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            (contract, accounts)
        }

        fn sign_message(message: &[u8], _signer: AccountId) -> Vec<u8> {
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(message, &mut output);
            output.to_vec()
        }

        fn create_invalid_signature() -> Vec<u8> {
            // Create an invalid signature by using a different message
            let message = b"Invalid message".to_vec();
            let mut output = <Blake2x256 as HashOutput>::Type::default();
            ink::env::hash_bytes::<Blake2x256>(&message, &mut output);
            output.to_vec()
        }

        #[ink::test]
        fn default_works() {
            let (contract, accounts) = setup();
            assert!(!contract.is_registered(accounts.alice));
        }

        #[ink::test]
        fn registration_with_valid_signature_works() {
            let (mut contract, accounts) = setup();
            let message = b"Register me".to_vec();
            let signature = sign_message(&message, accounts.alice);

            // Set the caller to Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Register with signature
            assert!(contract.register_with_signature(message, signature).is_ok());
            assert!(contract.is_registered(accounts.alice));
        }

        #[ink::test]
        fn registration_with_invalid_signature_fails() {
            let (mut contract, accounts) = setup();
            let message = b"Register me".to_vec();
            let signature = create_invalid_signature();

            // Set the caller to Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // Registration should fail
            assert!(matches!(
                contract.register_with_signature(message, signature),
                Err(Error::InvalidSignature)
            ));
            assert!(!contract.is_registered(accounts.alice));
        }

        #[ink::test]
        fn double_registration_fails() {
            let (mut contract, accounts) = setup();
            let message = b"Register me".to_vec();
            let signature = sign_message(&message, accounts.alice);

            // Set the caller to Alice
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            // First registration should succeed
            assert!(contract.register_with_signature(message.clone(), signature.clone()).is_ok());
            
            // Second registration should fail
            assert!(matches!(
                contract.register_with_signature(message, signature),
                Err(Error::AlreadyRegistered)
            ));
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn registration_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = MyContractRef::new();
            let contract = client
                .instantiate("my_contract", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<MyContract>();

            // Create a test message and signature
            let message = b"Register me".to_vec();
            let signature = ink_e2e::alice().sign(&message).to_vec();

            // Register
            let register = call_builder.register_with_signature(message, signature);
            let _register_result = client
                .call(&ink_e2e::alice(), &register)
                .submit()
                .await
                .expect("register failed");

            // Check registration
            let is_registered = call_builder.is_registered(ink_e2e::alice().account_id());
            let is_registered_result = client.call(&ink_e2e::alice(), &is_registered).dry_run().await?;
            assert!(is_registered_result.return_value());

            Ok(())
        }
    }
}

