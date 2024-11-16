const { Server } = require('stellar-sdk');

// exchange.js
class Exchange {
    async getTokenReserves() {
      return 100.0;
    }
  
    async getTokenAmount(amount) {
      return amount / 2;
    }
  }
  
  module.exports = { Exchange };