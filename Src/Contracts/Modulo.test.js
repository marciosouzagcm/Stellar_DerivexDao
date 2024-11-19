const { Exchange } = require('../Exchange');
const { expect } = require('chai');

describe('Exchange', () => {
  let exchange;

  beforeEach(() => {
    exchange = new Exchange();
  });

  it('should get token reserves', async () => {
    // Verifica se o método getTokenReserves retorna um número maior ou igual a 0.
    const reserves = await exchange.getTokenReserves();
    expect(reserves).to.be.a('number');
    expect(reserves).to.be.gte(0);
  });

  it('should get token amount', async () => {
    // Verifica se o método getTokenAmount retorna um número maior ou igual a 0.
    const amount = await exchange.getTokenAmount(10.0);
    expect(amount).to.be.a('number');
    expect(amount).to.be.gte(0);
  });

  it('should throw error on invalid input', async () => {
    // Verifica se o método getTokenAmount lança um erro em entrada inválida.
    try {
      await exchange.getTokenAmount('invalid');
      expect.fail();
    } catch (error) {
      expect(error).to.be.an('error');
    }
  });
});