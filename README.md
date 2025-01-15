##Stellar_DerivexDao

Stellar_DerivexDao é um projeto baseado na blockchain Stellar, focado na criação e gerenciamento de exchanges descentralizadas para tokens específicos. O projeto permite a criação de contas, a gestão de exchanges associadas a tokens e a interação com a rede Stellar de maneira segura e eficiente.

Funcionalidades
Criação de Conta: Através do método create_account, o projeto permite a criação de contas na blockchain Stellar, associando um saldo inicial ao processo.
Gerenciamento de Exchanges: O DerivexFactory gerencia um conjunto de exchanges, onde cada exchange é associada a um token específico. Ele permite a criação, consulta, remoção e associação de tokens a exchanges.
Operações com Exchanges e Tokens: É possível criar uma nova exchange associada a um token, buscar a exchange a partir do token e até remover uma exchange do sistema.
Segurança: O uso de chaves secretas e a validação de entradas (como tokens) garantem que as operações sejam seguras e válidas.
