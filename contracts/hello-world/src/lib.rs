#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec, Map};

const BALANCES: Symbol = symbol_short!("BAL");
const DIVIDENDS: Symbol = symbol_short!("DIV");

#[contracttype]
#[derive(Clone)]
pub struct TokenHolder {
    pub balance: u64,
    pub dividends: u64,
}

#[contract]
pub struct DividendToken;

#[contractimpl]
impl DividendToken {
    pub fn init_holder(env: Env, user: Address) {
        let mut balances: Map<Address, TokenHolder> = env.storage().instance().get(&BALANCES).unwrap_or(Map::new(&env));
        balances.set(user.clone(), TokenHolder { balance: 0, dividends: 0 });
        env.storage().instance().set(&BALANCES, &balances);
    }

    pub fn mint(env: Env, to: Address, amount: u64) {
        let mut balances: Map<Address, TokenHolder> = env.storage().instance().get(&BALANCES).unwrap_or(Map::new(&env));
        let mut holder = balances.get(to.clone()).unwrap_or(TokenHolder { balance: 0, dividends: 0 });
        holder.balance += amount;
        balances.set(to.clone(), holder);
        env.storage().instance().set(&BALANCES, &balances);
    }

    pub fn distribute_dividends(env: Env, amount: u64) {
        let mut balances: Map<Address, TokenHolder> = env.storage().instance().get(&BALANCES).unwrap_or(Map::new(&env));
        let total_supply: u64 = balances.values().into_iter().map(|h| h.balance).sum();
        for (address, mut holder) in balances.iter() {
            let share = (holder.balance as u128 * amount as u128 / total_supply as u128) as u64;
            holder.dividends += share;
            balances.set(address, holder);
        }
        env.storage().instance().set(&BALANCES, &balances);
    }

    pub fn view_holder(env: Env, user: Address) -> TokenHolder {
        let balances: Map<Address, TokenHolder> = env.storage().instance().get(&BALANCES).unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(TokenHolder { balance: 0, dividends: 0 })
    }
}
