const Token = require('../Token');

describe('Token', () => {
  let token;

  beforeEach(() => {
    token = new Token('SVX', 'GBBD47IF6NO3HIYTQAV5ZNO4CZMQVPPRL7SS4N4WEH5SVJ2T7E7QY');
  });

  it('should criar um novo token', async () => {
    expect(token.code).to.equal('SVX');
    expect(token.issuer).to.equal('GBBD47IF6NO3HIYTQAV5ZNO4CZMQVPPRL7SS4N4WEH5SVJ2T7E7QY');
  });

  it('should verificar se o token é válido', async () => {
    expect(token.isValid()).to.be.true;
  });

  it('should lançar erro ao criar token inválido', async () => {
    try {
      new Token('', '');
      expect.fail();
    } catch (error) {
      expect(error).to.be.an('error');
    }
  });
});