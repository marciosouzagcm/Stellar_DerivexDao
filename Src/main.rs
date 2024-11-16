mod dao;

fn main() {
    // Cria uma nova instância do DAO
    let dao = dao::StellarDerivexDAO::new();

    // Executa o método executar do DAO
    dao.executar();

    println!("Stellar Derivex DAO iniciado com sucesso!");
}