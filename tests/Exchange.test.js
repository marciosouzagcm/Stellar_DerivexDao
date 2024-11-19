const Exchange = require('./../../../src/Exchange');

describe('Exchange', () => {
  let exchange;

  beforeEach(() => {
    exchange = new Exchange();
  });

  it('should obter as reserves de tokens', async () => {
    // Verifica se o método getTokenReserves retorna um número maior ou igual a 0.
    const reserves = await exchange.getTokenReserves();
    expect(reserves).to.be.a('number');
    expect(reserves).to.be.gte(0);
  });

  it('should calcular a quantidade de tokens', async () => {
    // Verifica se o método getTokenAmount retorna um número maior ou igual a 0.
    const amount = await exchange.getTokenAmount(10.0);
    expect(amount).to.be.a('number');
    expect(amount).to.be.gte(0);
  });

  it('should lançar erro em entrada inválida', async () => {
    // Verifica se o método getTokenAmount lança um erro em entrada inválida.
    try {
      await exchange.getTokenAmount('invalid');
      expect.fail();
    } catch (error) {
      expect(error).to.be.an('error');
    }
  });
});