const { Asset, Server } = require('stellar-sdk');
const { expect } = require('chai');


class Token {
  constructor(publicKey, symbol, server) {
    this.publicKey = publicKey;
    this.symbol = symbol;
    this.server = server;
  }

  async getBalance() {
    try {
      const account = await this.server.loadAccount(this.publicKey);
      const balance = account.balances.find((balance) => balance.asset_type === 'credit_alphanum4' && balance.asset_code === this.symbol);
      return balance ? balance.balance : 0;
    } catch (error) {
      throw error;
    }
  }

  async createEmission(amount) {
    try {
      const transaction = new Stellar.TransactionBuilder(this.server.loadAccount(this.publicKey))
        .addOperation(Stellar.Operation.createAsset({
          asset: new Asset(this.symbol, this.publicKey),
          amount,
        }))
        .build();
      await this.server.submitTransaction(transaction);
      return true;
    } catch (error) {
      throw error;
    }
  }

  async transfer(destination, amount) {
    try {
      const transaction = new Stellar.TransactionBuilder(this.server.loadAccount(this.publicKey))
        .addOperation(Stellar.Operation.payment({
          destination,
          asset: new Asset(this.symbol, this.publicKey),
          amount,
        }))
        .build();
      await this.server.submitTransaction(transaction);
      return true;
    } catch (error) {
      throw error;
    }
  }
}

module.exports = Token;