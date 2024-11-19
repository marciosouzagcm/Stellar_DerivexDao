use stellar_rust_sdk::{ 
    Account, 
    Asset, 
    Keypair, 
    Network, 
    Server, 
    TransactionBuilder, 
};
use stellar_rust_sdk::transaction::{Transaction, Operation};
use std::error::Error;

const HORIZON_URL: &str = "https://horizon.stellar.org";
const NETWORK_PASSPHRASE: &str = "Test Network";

pub struct Token {
    public_key: String,
    symbol: String,
    server: Server,
}

impl Token {
    pub fn new(public_key: String, symbol: String) -> Self {
        let server = Server::new(HORIZON_URL);
        Token {
            public_key,
            symbol,
            server,
        }
    }

    pub async fn get_balance(&self) -> Result<f64, Box<dyn Error>> {
        let account = self.server.load_account(&self.public_key).await?;
        let balance = account
            .balances
            .iter()
            .find(|b| b.asset_type == "credit_alphanum4" && b.asset_code == self.symbol);
        match balance {
            Some(b) => Ok(b.balance),
            None => Ok(0.0),
        }
    }

    pub async fn create_emission(&self, amount: f64) -> Result<(), Box<dyn Error>> {
        if amount <= 0.0 {
            return Err("Quantidade deve ser um número positivo".into());
        }
        let source_account = self.server.load_account(&self.public_key).await?;
        let transaction = TransactionBuilder::new(&source_account, &Network::test_network(), 100)
            .append_operation(Operation::CreateAsset {
                code: self.symbol.clone(),
                issuer: self.public_key.clone(),
                max_supply: 1000.0,
            })
            .build()?;
        self.server.submit_transaction(&transaction).await?;
        Ok(())
    }

    pub async fn transfer(&self, destination: String, amount: f64) -> Result<(), Box<dyn Error>> {
        if amount <= 0.0 {
            return Err("Quantidade deve ser um número positivo".into());
        }
        let source_account = self.server.load_account(&self.public_key).await?;
        let transaction = TransactionBuilder::new(&source_account, &Network::test_network(), 100)
            .append_operation(Operation::Payment {
                destination,
                amount,
                asset: Asset::new(self.symbol.clone(), self.public_key.clone()),
            })
            .build()?;
        self.server.submit_transaction(&transaction).await?;
        Ok(())
    }
}