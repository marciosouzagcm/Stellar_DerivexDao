const stellar = require('stellar-sdk');
const Server = stellar.Server;

class Token {
  /**
   * Construtor da classe Token.
   * 
   * @param {string} publicKey - Chave pública do emissor do token.
   * @param {string} symbol - Símbolo do token.
   * @param {Server} server - Instância do servidor Stellar.
   */
  constructor(publicKey, symbol, server) {
    if (typeof publicKey !== 'string') {
      throw new Error('Public key deve ser uma string');
    }
    if (typeof symbol !== 'string') {
      throw new Error('Símbolo deve ser uma string');
    }
    if (!(server instanceof Server)) {
      throw new Error('Servidor deve ser uma instância de Server');
    }
    this.publicKey = publicKey;
    this.symbol = symbol;
    this.server = server;
  }

  /**
   * Obtem o saldo do token para o emissor.
   * 
   * @returns {Promise<number>} Saldo do token.
   */
  async getBalance() {
    try {
      const account = await this.server.loadAccount(this.publicKey);
      if (!account) {
        throw new Error('Conta não encontrada');
      }
      const balance = account.balances.find((balance) => balance.asset_type === 'credit_alphanum4' && balance.asset_code === this.symbol);
      return balance ? balance.balance : 0;
    } catch (error) {
      throw error;
    }
  }

  /**
   * Cria uma nova emissão do token.
   * 
   * @param {number} amount - Quantidade de tokens a serem emitidos.
   * @returns {Promise<boolean>} True se a emissão for bem-sucedida.
   */
  async createEmission(amount) {
    if (typeof amount !== 'number' || amount <= 0) {
      throw new Error('Quantidade deve ser um número positivo');
    }
    try {
      const transaction = new TransactionBuilder(this.server.loadAccount(this.publicKey))
        .addOperation(Transaction.Operation.createAsset({
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

  /**
   * Transfere tokens para um destinatário.
   * 
   * @param {string} destination - Chave pública do destinatário.
   * @param {number} amount - Quantidade de tokens a serem transferidos.
   * @returns {Promise<boolean>} True se a transferência for bem-sucedida.
   */
  async transfer(destination, amount) {
    if (typeof destination !== 'string') {
      throw new Error('Destinatário deve ser uma string');
    }
    if (typeof amount !== 'number' || amount <= 0) {
      throw new Error('Quantidade deve ser um número positivo');
    }
    try {
      const transaction = new TransactionBuilder(this.server.loadAccount(this.publicKey))
        .addOperation(Transaction.Operation.payment({
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