#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec, log};

#[contracttype]
#[derive(Clone)]
pub struct ScholarshipRecord {
    pub student: Address,
    pub amount: i128,
    pub description: soroban_sdk::String,
}

#[contract]
pub struct ScholarshipFund;

#[contractimpl]
impl ScholarshipFund {
    // 1. Khởi tạo hợp đồng với địa chỉ Token (ví dụ: USDC) và người quản trị
    pub fn init(env: Env, admin: Address, token_wasm_hash: Address) {
        if env.storage().instance().has(&soroban_sdk::symbol_short!("admin")) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&soroban_sdk::symbol_short!("admin"), &admin);
        env.storage().instance().set(&soroban_sdk::symbol_short!("token"), &token_wasm_hash);
    }

    // 2. Quyên góp tiền vào quỹ
    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();
        
        let token_addr: Address = env.storage().instance().get(&soroban_sdk::symbol_short!("token")).unwrap();
        let client = token::Client::new(&env, &token_addr);
        
        // Chuyển token từ người quyên góp vào hợp đồng này
        client.transfer(&donor, &env.current_contract_address(), &amount);
        
        log!(&env, "Donation received: {}", amount);
    }

    // 3. Phát hành học bổng (Chỉ admin mới có quyền)
    pub fn issue(env: Env, student: Address, amount: i128, description: soroban_sdk::String) {
        let admin: Address = env.storage().instance().get(&soroban_sdk::symbol_short!("admin")).unwrap();
        admin.require_auth();

        let token_addr: Address = env.storage().instance().get(&soroban_sdk::symbol_short!("token")).unwrap();
        let client = token::Client::new(&env, &token_addr);

        // Chuyển token từ quỹ đến sinh viên
        client.transfer(&env.current_contract_address(), &student, &amount);

        // Lưu lịch sử để theo dõi minh bạch
        let mut history: Vec<ScholarshipRecord> = env
            .storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("history"))
            .unwrap_or(Vec::new(&env));

        history.push_back(ScholarshipRecord {
            student,
            amount,
            description,
        });

        env.storage().instance().set(&soroban_sdk::symbol_short!("history"), &history);
    }

    // 4. Xem lịch sử giải ngân học bổng
    pub fn get_history(env: Env) -> Vec<ScholarshipRecord> {
        env.storage()
            .instance()
            .get(&soroban_sdk::symbol_short!("history"))
            .unwrap_or(Vec::new(&env))
    }
}