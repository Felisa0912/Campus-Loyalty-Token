```rust
#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn reward_student() {
    let env = Env::default();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let student = Address::generate(&env);

    client.initialize(&admin);

    env.mock_all_auths();

    client.reward(&admin, &student, &100);

    let balance = client.balance(&student);

    assert_eq!(balance, 100);
}
```
