// exchange.test.js
const { Exchange } = require('../src/exchange');
const { expect } = require('chai');


describe('Exchange', () => {
  it('should get token reserves', async () => {
    const exchange = new Exchange();
    const reserves = await exchange.getTokenReserves();
    expect(reserves).toBe(0);
  });

  it('should get token amount', async () => {
    const exchange = new Exchange();
    const amount = await exchange.getTokenAmount(10.0);
    expect(amount).toBe(0);
  });
});