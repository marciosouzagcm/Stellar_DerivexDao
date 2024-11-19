const assert = require('assert');
const { Server } = require('stellar-sdk');
const DerivexFactory = require('../../tests/derivex-factory');
const { expect } = require('chai');


describe('DerivexFactory', () => {
  it('should criar uma nova exchange', async () => {
    const server = new Server('https://horizon.stellar.org');
    const factory = new DerivexFactory(server);
    const tokenAddress = 'GBBD47IF6NO3HIYTQAV5ZNO4CZMQVPPRL7SS4N4WEH5SVJ2T7E7QY';
    const exchange = await factory.createNewExchange(tokenAddress);
    assert.ok(exchange);
  });

  it('should consultar uma exchange existente', async () => {
    const server = new Server('(link unavailable)');
    const factory = new DerivexFactory(server);
    const tokenAddress = 'GBBD47IF6NO3HIYTQAV5ZNO4CZMQVPPRL7SS4N4WEH5SVJ2T7E7QY';
    const exchange = await factory.getExchange(tokenAddress);
    assert.ok(exchange);
  });
});