#![cfg(test)]

use super::*; // Asegúrate de que el contrato está siendo importado correctamente
use soroban_sdk::{vec, Env, String};

#[test]
fn test_primo_contract() {
    let env = Env::default();
    let contract_id = env.register(PrimoContract, ()); // Registra el contrato
    let client = PrimoContractClient::new(&env, &contract_id); // Crea un cliente para el contrato

    // Test con número negativo
    let resultado_negativo = client.verificar_numero(&-5); // Solo pasamos el número
    assert_eq!(resultado_negativo, vec![
        &env,
        String::from_str(&env, "0"),
    ]);

    // Test con número menor que 2
    let resultado_menor_que_dos = client.verificar_numero(&1); // Solo pasamos el número
    assert_eq!(resultado_menor_que_dos, vec![
        &env,
        String::from_str(&env, "El número 1 no es primo."),
    ]);

    // Test con número primo
    let resultado_primo = client.verificar_numero(&7); // Solo pasamos el número
    assert_eq!(resultado_primo, vec![
        &env,
        String::from_str(&env, "El número 7 es primo."),
    ]);

    // Test con número no primo (ejemplo: 8)
    let resultado_no_primo = client.verificar_numero(&8); // Solo pasamos el número
    assert_eq!(resultado_no_primo, vec![
        &env, 
        String::from_str(&env, "El número 8 no es primo."),
        String::from_str(&env, "8"),
        String::from_str(&env, "16"),
        String::from_str(&env, "24"),
        String::from_str(&env, "32"),
        String::from_str(&env, "40")
    ]);
}
