// Token.js
const { Server } = require('@stellar/stellar-sdk');
const { Server } = require('stellar-sdk');

class Token {
  constructor(publicKey, symbol, server) {
    this.publicKey = publicKey;
    this.symbol = symbol;
    this.server = server;
  }

  async getBalance() {
    const account = await this.server.loadAccount(this.publicKey);
    const balance = account.balances.find((balance) => balance.asset_type === 'credit_alphanum4' && balance.asset_code === this.symbol);
    return balance ? balance.balance : 0;
  }

  async createEmission(amount) {
    const transaction = new Stellar.TransactionBuilder(this.server.loadAccount(this.publicKey))
      .addOperation(Stellar.Operation.createAsset({
        asset: new Asset(this.symbol, this.publicKey),
        amount,
      }))
      .build();
    await this.server.submitTransaction(transaction);
    return true;
  }

  async transfer(destination, amount) {
    const transaction = new Stellar.TransactionBuilder(this.server.loadAccount(this.publicKey))
      .addOperation(Stellar.Operation.payment({
        destination,
        asset: new Asset(this.symbol, this.publicKey),
        amount,
      }))
      .build();
    await this.server.submitTransaction(transaction);
    return true;
  }
}

module.exports = Token;