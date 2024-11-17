#[cfg(test)]
mod tests {
    use super::*;
    use stellar_derivex_dao::dao::StellarDerivexDAO;

    #[test]
    fn test_criar_par_de_chaves() {
        let dao = StellarDerivexDAO::new().unwrap();
        let _account_id = dao.criar_par_de_chaves().unwrap();
    }

    #[test]
    fn test_criar_conta() {
        let dao = StellarDerivexDAO::new().unwrap();
        let account_id = dao.criar_par_de_chaves().unwrap();
        let _account = dao.criar_conta(&account_id).unwrap();
    }

    #[test]
    fn test_criar_transacao() {
        let dao = StellarDerivexDAO::new().unwrap();
        let account_id = dao.criar_par_de_chaves().unwrap();
        let sc_address = dao.criar_token().unwrap();
        let asset = dao.criar_token().unwrap();
        let _transaction = dao.criar_transacao(&account_id, &sc_address, &asset).unwrap();
    }

    #[test]
    fn test_assinar_transacao() {
        let dao = StellarDerivexDAO::new().unwrap();
        let account_id = dao.criar_par_de_chaves().unwrap();
        let sc_address = dao.criar_token().unwrap();
        let asset = dao.criar_token().unwrap();
        let transaction = dao.criar_transacao(&account_id, &sc_address, &asset).unwrap();
        let _signed_transaction = dao.assinar_transacao(&transaction, &account_id).unwrap();
    }

    #[test]
    fn test_enviar_transacao() {
        let dao = StellarDerivexDAO::new().unwrap();
        let account_id = dao.criar_par_de_chaves().unwrap();
        let sc_address = dao.criar_token().unwrap();
        let asset = dao.criar_token().unwrap();
        let transaction = dao.criar_transacao(&account_id, &sc_address, &asset).unwrap();
        let signed_transaction = dao.assinar_transacao(&transaction, &account_id).unwrap();
        dao.enviar_transacao(&signed_transaction).unwrap();
    }
}