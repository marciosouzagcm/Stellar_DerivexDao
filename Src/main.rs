use stellar_derivex_dao::dao::StellarDerivexDAO;

fn main() -> Result<(), anyhow::Error> {
    let dao = StellarDerivexDAO::new()
        .map_err(|e| anyhow::anyhow!("Erro ao criar DAO: {}", e))?;

    println!("Stellar Derivex DAO iniciado com sucesso!");

    dao.criar_par_de_chaves()
        .map_err(|e| anyhow::anyhow!("Erro ao criar par de chaves: {}", e))?;

    Ok(())
}