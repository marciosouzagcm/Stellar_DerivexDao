/// Módulo para gerenciar exchanges e tokens na rede Stellar.

use stellar_rust_sdk::{Exchange, Server};
use std::collections::HashMap;
use std::error::Error;

/// Definição da classe DerivexFactory.
///
/// A classe DerivexFactory é responsável por gerenciar exchanges e tokens na rede Stellar.
pub struct DerivexFactory {
    /// Servidor Stellar.
    server: Server,
    /// Mapeamento de tokens para exchanges.
    token_to_exchange: HashMap<String, Exchange>,
    /// Mapeamento de exchanges para tokens.
    exchange_to_token: HashMap<Exchange, String>,
    /// Mapeamento de IDs para tokens.
    id_to_token: HashMap<u64, String>,
}

impl DerivexFactory {
    /// Cria uma nova instância de DerivexFactory.
    ///
    /// # Parâmetros
    ///
    /// * `server`: Servidor Stellar.
    ///
    /// # Retorno
    ///
    /// * `DerivexFactory`: Nova instância de DerivexFactory.
    pub fn new(server: Server) -> Self {
        DerivexFactory {
            server,
            token_to_exchange: HashMap::new(),
            exchange_to_token: HashMap::new(),
            id_to_token: HashMap::new(),
        }
    }

    /// Cria uma nova exchange associada ao token.
    ///
    /// # Parâmetros
    ///
    /// * `token_address`: Endereço do token.
    ///
    /// # Retorno
    ///
    /// * `Exchange`: Nova exchange criada.
    ///
    /// # Erro
    ///
    /// * `Box<dyn Error>`: Erro ao criar a exchange.
    pub fn create_new_exchange(&mut self, token_address: String) -> Result<Exchange, Box<dyn Error>> {
        // Validação de dados.
        if token_address.is_empty() {
            return Err("Endereço de token inválido".into());
        }
        
        // Criação da exchange.
        let exchange = Exchange::new(&self.server, &token_address)?;
        
        // Armazenamento da exchange.
        self.token_to_exchange.insert(token_address.clone(), exchange.clone());
        self.exchange_to_token.insert(exchange.clone(), token_address.clone());
        
        Ok(exchange)
    }

    /// Consulta a exchange associada a um token.
    ///
    /// # Parâmetros
    ///
    /// * `token_address`: Endereço do token.
    ///
    /// # Retorno
    ///
    /// * `Option<&Exchange>`: Exchange associada ao token, se existir.
    pub fn get_exchange(&self, token_address: &str) -> Option<&Exchange> {
        self.token_to_exchange.get(token_address)
    }

    /// Consulta o token associado a uma exchange.
    ///
    /// # Parâmetros
    ///
    /// * `exchange`: Exchange.
    ///
    /// # Retorno
    ///
    /// * `Option<&String>`: Token associado à exchange, se existir.
    pub fn get_token(&self, exchange: &Exchange) -> Option<&String> {
        self.exchange_to_token.get(exchange)
    }

    /// Consulta o token associado a um ID específico.
    ///
    /// # Parâmetros
    ///
    /// * `token_id`: ID do token.
    ///
    /// # Retorno
    ///
    /// * `Option<&String>`: Token associado ao ID, se existir.
    pub fn get_token_with_id(&self, token_id: u64) -> Option<&String> {
        self.id_to_token.get(&token_id)
    }
}

fn main() {
    // Criação do servidor Stellar.
    let server = Server::new("(link unavailable)");
    
    // Criação da fábrica de exchanges.
    let mut factory = DerivexFactory::new(server);
    
    // Criação de uma nova exchange.
    let exchange = factory.create_new_exchange("TokenAddress".to_string()).unwrap();
    
    // Consulta da exchange associada ao token.
    let exchange_consultada = factory.get_exchange("TokenAddress");
    println!("{:?}", exchange_consultada);
}
