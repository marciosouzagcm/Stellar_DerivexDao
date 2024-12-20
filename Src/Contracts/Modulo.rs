/// Módulo para gerenciar exchanges na rede Stellar.

use stellar_rust_sdk::{Server, Asset, Account};
use std::error::Error;

/// Definição da classe Exchange.
///
/// A classe Exchange é responsável por gerenciar a troca de tokens e XLM na rede Stellar.
pub struct Exchange {
    /// Endereço do token.
    token_address: String,
    /// Endereço do contrato factory.
    factory_address: String,
    /// Servidor Stellar.
    server: Server,
}

impl Exchange {
    /// Cria uma nova instância de Exchange.
    ///
    /// # Parâmetros
    ///
    /// * `token_address`: Endereço do token.
    /// * `factory_address`: Endereço do contrato factory.
    /// * `server`: Servidor Stellar.
    ///
    /// # Retorno
    ///
    /// * `Exchange`: Nova instância de Exchange.
    pub fn new(token_address: String, factory_address: String, server: Server) -> Self {
        Exchange {
            token_address,
            factory_address,
            server,
        }
    }

    /// Consulta o saldo de tokens na pool.
    ///
    /// # Retorno
    ///
    /// * `Result<f64, Box<dyn Error>>`: Saldo de tokens na pool.
    pub fn get_token_reserves(&self) -> Result<f64, Box<dyn Error>> {
        // Criação do ativo.
        let asset = Asset::new(&self.token_address);
        
        // Carregamento do saldo da conta.
        let balance = self.server.load_account(&self.factory_address)?.balance(&asset)?;
        
        Ok(balance)
    }

    /// Calcula quantos tokens o utilizador recebe ao vender XLM.
    ///
    /// # Parâmetros
    ///
    /// * `xlm_sold`: Quantidade de XLM vendida.
    ///
    /// # Retorno
    ///
    /// * `f64`: Quantidade de tokens recebida.
    pub fn get_token_amount(&self, xlm_sold: f64) -> f64 {
        // Consulta do saldo de tokens na pool.
        let output_reserve = self.get_token_reserves().unwrap_or(0.0);
        
        // Cálculo da quantidade de tokens recebida.
        let input_amount_with_fee = xlm_sold * 0.997;
        let numerator = input_amount_with_fee * output_reserve;
        let denominator = (output_reserve * 1000.0) + input_amount_with_fee;
        numerator / denominator
    }

    /// Calcula quanto XLM o utilizador recebe ao vender tokens.
    ///
    /// # Parâmetros
    ///
    /// * `tokens_sold`: Quantidade de tokens vendida.
    ///
    /// # Retorno
    ///
    /// * `f64`: Quantidade de XLM recebida.
    pub fn get_xlm_amount(&self, tokens_sold: f64) -> f64 {
        // Consulta do saldo de tokens na pool.
        let input_reserve = self.get_token_reserves().unwrap_or(0.0);
        
        // Cálculo da quantidade de XLM recebida.
        let output_amount = tokens_sold * 0.997;
        let numerator = output_amount * input_reserve;
        let denominator = (input_reserve * 1000.0) + output_amount;
        numerator / denominator
    }

    /// Permite que os utilizadores adicionem liquidez à pool.
    ///
    /// # Parâmetros
    ///
    /// * `tokens_added`: Quantidade de tokens adicionada.
    ///
    /// # Retorno
    ///
    /// * `f64`: Quantidade de XLM adicionada.
    pub fn add_liquidity(&self, tokens_added: f64) -> f64 {
        // Carregamento do saldo da conta.
        let xlm_balance = self.server.load_account(&self.factory_address).unwrap().balance;
        
        // Consulta do saldo de tokens na pool.
        let token_balance = self.get_token_reserves().unwrap_or(0.0);
        
        // Cálculo da quantidade de XLM adicionada.
        if token_balance == 0.0 {
            xlm_balance
        } else {
            (xlm_balance * tokens_added) / token_balance
        }
    }

    /// Permite que os utilizadores removam liquidez da pool.
    ///
    /// # Parâmetros
    ///
    /// * `liquidity_removed`: Quantidade de liquidez removida.
    ///
    /// # Retorno
    ///
    /// * `(f64, f64)`: Quantidade de XLM e tokens removidos.
    pub fn remove_liquidity(&self, liquidity_removed: f64) -> (f64, f64)