const assert = require('assert');
const { Server } = require('stellar-sdk');
const Exchange = require('../Src/Exchange');
const { expect } = require('chai');


describe('Exchange', () => {
  let server;
  let exchange;

  beforeEach(async () => {
    server = new Server('https://horizon.stellar.org');
    exchange = new Exchange(
      'GBBD47IF6NO3HIYTQAV5ZNO4CZMQVPPRL7SS4N4WEH5SVJ2T7E7QY',
      'GABCD1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ',
      server
    );
  });

  it('should obter as reserves de tokens', async () => {
    const reserves = await exchange.getTokenReserves();
    assert.ok(reserves > 0);
  });

  it('should calcular a quantidade de tokens', async () => {
    const tokenAmount = await exchange.getTokenAmount(100);
    assert.ok(tokenAmount > 0);
  });

  it('should adicionar liquidez', async () => {
    const liquidity = await exchange.addLiquidity(100);
    assert.ok(liquidity > 0);
  });
});