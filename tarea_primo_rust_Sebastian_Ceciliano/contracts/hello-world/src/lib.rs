#![no_std]
extern crate alloc;
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};
use alloc::string::ToString;
#[contract]
pub struct PrimoContract;

// This is a sample contract. Replace this placeholder with your own contract logic.
// A corresponding test example is available in `test.rs`.
//
// For comprehensive examples, visit <https://github.com/stellar/soroban-examples>.
// The repository includes use cases for the Stellar ecosystem, such as data storage on
// the blockchain, token swaps, liquidity pools, and more.
//
// Refer to the official documentation:
// <https://developers.stellar.org/docs/build/smart-contracts/overview>.

#[contractimpl]
impl PrimoContract {
    pub fn verificar_numero(env: Env, numero: i32) -> Vec<String> {
        if numero < 0 {
            return vec![&env, String::from_str(&env, "0")];
        }
        if numero < 2 {
            return vec![&env, String::from_str(&env, &("El número ".to_string() + &numero.to_string() + " no es primo."))];
        }

        for i in 2..=((numero as f64).sqrt() as i32) {
            if numero % i == 0 {
                let mut respuesta = vec![&env, String::from_str(&env, &("El número ".to_string() + &numero.to_string() + " no es primo."))];
                let multiplos = Self::obtener_multiplos(&env, numero);
                respuesta.append(&multiplos);
                return respuesta;
            }
        }

        vec![&env, String::from_str(&env, &("El número ".to_string() + &numero.to_string() + " es primo."))]
    }

    fn obtener_multiplos(env: &Env, numero: i32) -> Vec<String> {
        let mut multiplos = Vec::new(env);
        for i in 1..=5 {
            multiplos.push_back(String::from_str(env, &(numero * i).to_string()));
        }
        multiplos
    }
}

mod test;
