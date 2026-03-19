#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, String, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct DomainRecord {
    pub owner: Address,
    pub ip: String,
}

#[contract]
pub struct DecentralizedDNS;

#[contractimpl]
impl DecentralizedDNS {

    // Register a new domain
    pub fn register(env: Env, domain: Symbol, owner: Address, ip: String) {
        let mut domains: Map<Symbol, DomainRecord> =
            env.storage().instance().get(&symbol_short!("DOMAINS")).unwrap_or(Map::new(&env));

        // Check if domain already exists
        if domains.contains_key(domain.clone()) {
            panic!("Domain already registered");
        }

        let record = DomainRecord {
            owner: owner.clone(),
            ip,
        };

        domains.set(domain, record);
        env.storage().instance().set(&symbol_short!("DOMAINS"), &domains);
    }

    // Resolve domain → IP
    pub fn resolve(env: Env, domain: Symbol) -> String {
        let domains: Map<Symbol, DomainRecord> =
            env.storage().instance().get(&symbol_short!("DOMAINS")).unwrap();

        let record = domains.get(domain).unwrap();
        record.ip
    }

    // Update IP (only owner can update)
    pub fn update(env: Env, domain: Symbol, owner: Address, new_ip: String) {
        let mut domains: Map<Symbol, DomainRecord> =
            env.storage().instance().get(&symbol_short!("DOMAINS")).unwrap();

        let mut record = domains.get(domain.clone()).unwrap();

        if record.owner != owner {
            panic!("Not authorized");
        }

        record.ip = new_ip;
        domains.set(domain, record);

        env.storage().instance().set(&symbol_short!("DOMAINS"), &domains);
    }

    // Get owner of a domain
    pub fn get_owner(env:
         Env, domain: Symbol) -> Address {
        let domains: Map<Symbol, DomainRecord> =
            env.storage().instance().get(&symbol_short!("DOMAINS")).unwrap();

        let record = domains.get(domain).unwrap();
        record.owner
    }
}