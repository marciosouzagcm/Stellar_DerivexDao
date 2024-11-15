pub mod dao;

// Arquivo src/(link unavailable)

mod dao; // Importa o módulo dao

fn main() {
    // Inicializa o DAO
    let dao = dao::StellarDerivexDAO::new();

    // Executa alguma ação no DAO
    dao.executar();

    println!("Stellar Derivex DAO iniciado com sucesso!");
}