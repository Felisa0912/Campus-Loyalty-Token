```rust
#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env,
};

#[contract]
pub struct Contract;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contractimpl]
impl Contract {
    // Khởi tạo admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    // Thưởng điểm cho sinh viên
    pub fn reward(
        env: Env,
        admin: Address,
        student: Address,
        amount: u32,
    ) {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        if admin != stored_admin {
            panic!("not admin");
        }

        let key = DataKey::Balance(student);

        let current: u32 = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&key, &(current + amount));
    }

    // Xem số điểm
    pub fn balance(env: Env, student: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(student))
            .unwrap_or(0)
    }
}
```
