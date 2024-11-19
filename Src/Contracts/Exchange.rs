use stellar_rust_sdk::{Server, Asset, Account};
use std::error::Error;

/// Módulo para gerenciar exchanges na rede Stellar.

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
    /// * `token_address`: Endereço do token.
    /// * `factory_address`: Endereço do contrato factory.
    /// * `server`: Servidor Stellar.
    /// 
    /// # Retorno
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
    /// * `Result<f64, Box<dyn Error>>`: Saldo de tokens na pool.
    pub fn get_token_reserves(&self) -> Result<f64, Box<dyn Error>> {
        // Criação do ativo.
        let asset = Asset::new(&self.token_address)?;
        
        // Carregamento do saldo da conta.
        let balance = self.server.load_account(&self.factory_address)?;
        
        // Verificação de erro.
        let balance = balance.balance(&asset).ok_or_else(|| "Saldo não encontrado")?;
        
        Ok(balance)
    }

    /// Calcula quantos tokens o utilizador recebe ao vender XLM.
    /// 
    /// # Parâmetros
    /// * `xlm_sold`: Quantidade de XLM vendida.
    /// 
    /// # Retorno
    /// * `f64`: Quantidade de tokens recebida.
    pub fn get_token_amount(&self, xlm_sold: f64) -> Result<f64, Box<dyn Error>> {
        if xlm_sold == 0.0 {
            return Err("Quantidade de XLM vendida não pode ser zero".into());
        }
        
        // Consulta do saldo de tokens na pool.
        let output_reserve = self.get_token_reserves()?;
        
        // Cálculo da quantidade de tokens recebida.
        let input_amount_with_fee = xlm_sold * 0.997;
        let numerator = input_amount_with_fee * output_reserve;
        let denominator = (output_reserve * 1000.0) + input_amount_with_fee;
        
        Ok(numerator / denominator)
    }

    /// Calcula quanto XLM o utilizador recebe ao vender tokens.
    /// 
    /// # Parâmetros
    /// * `tokens_sold`: Quantidade de tokens vendida.
    /// 
    /// # Retorno
    /// * `f64`: Quantidade de XLM recebida.
    pub fn get_xlm_amount(&self, tokens_sold: f64) -> Result<f64, Box<dyn Error>> {
        if tokens_sold == 0.0 {
            return Err("Quantidade de tokens vendida não pode ser zero".into());
        }
        
        // Consulta do saldo de tokens na pool.
        let input_reserve = self.get_token_reserves()?;
        
        // Cálculo da quantidade de XLM recebida.
        let output_amount = tokens_sold * 0.997;
        let numerator = output_amount * input_reserve;
        let denominator = (input_reserve * 1000.0) + output_amount;
        
        Ok(numerator / denominator)
    }

    /// Permite que os utilizadores adicionem liquidez à pool.
    /// 
    /// # Parâmetros
    /// * `tokens_added`: Quantidade de tokens adicionada.
    /// 
    /// # Retorno
    /// * `f64`: Quantidade de XLM adicionada.
    pub fn add_liquidity(&self, tokens_added: f64) -> Result<f64, Box<dyn Error>> {
        if tokens_added == 0.0 {
            return Err("Quantidade de tokens adicionada não pode ser zero".into());
        }
    // Carregamento do saldo da conta.
        let xlm_balance = self.server.load_account(&self.factory_address)?.balance;
        
        // Consulta do saldo de tokens na pool.
        let token_balance = self.get_token_reserves()?;
        
        // Cálculo da quantidade de XLM adicionada.
        if token_balance == 0.0 {
            Ok(xlm_balance)
        } else {
            Ok((xlm_balance * tokens_added)
        } else {
            Ok((xlm_balance * tokens_added) / token_balance)
        }
    }
}

/// Permite que os utilizadores removam liquidez da pool.
/// 
/// # Parâmetros
/// * `liquidity_removed`: Quantidade de liquidez removida.
/// 
/// # Retorno
/// * `(f64, f64)`: Quantidade de XLM e tokens removidos.
pub fn remove_liquidity(&self, liquidity_removed: f64) -> Result<(f64, f64), Box<dyn Error>> {
    if liquidity_removed == 0.0 {
        return Err("Quantidade de liquidez removida não pode ser zero".into());
    }
    
    // Consulta do saldo de tokens na pool.
    let token_balance = self.get_token_reserves()?;
    
    // Cálculo da quantidade de XLM removida.
    let xlm_removed = liquidity_removed * (token_balance / (token_balance + liquidity_removed));
    
    // Cálculo da quantidade de tokens removidos.
    let tokens_removed = liquidity_removed * (token_balance / (token_balance + liquidity_removed));
    
    Ok((xlm_removed, tokens_removed))
}
}

#[cfg(test)]
mod tests {
use super::*;
use stellar_rust_sdk::Server;

#[test]
fn test_get_token_reserves() {
    let server = Server::new ("https://horizon.stellar.org");
    let exchange = Exchange::new("Token".to_string(), "Factory".to_string(), server);
    assert!(exchange.get_token_reserves().is_ok());
}

#[test]
fn test_get_token_amount() {
    let server = Server::new("https://horizon.stellar.org");
    let exchange = Exchange::new("Token".to_string(), "Factory".to_string(), server);
    assert!(exchange.get_token_amount(100.0).is_ok());
}

#[test]
fn test_get_xlm_amount() {
    let server = Server::new("https://horizon.stellar.org");
    let exchange = Exchange::new("Token".to_string(), "Factory".to_string(), server);
    assert!(exchange.get_xlm_amount(100.0).is_ok());
}

#[test]
fn test_add_liquidity() {
    let server = Server::new("    const server = new Server ("https://horizon.stellar.org");
    let exchange = Exchange::new("Token".to_string(), "Factory".to_string(), server);
    assert!(exchange.add_liquidity(100.0).is_ok());
}

#[test]
fn test_remove_liquidity() {
    let server = Server::new ("https://horizon.stellar.org");
    let exchange = Exchange::new("Token".to_string(), "Factory".to_string(), server);
    assert!(exchange.remove_liquidity(100.0).is_ok());
}
}
       