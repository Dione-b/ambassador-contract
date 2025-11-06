# Ambassador Contract

Um smart contract Soroban para gerenciamento de presença e perfis de embaixadores na blockchain Stellar.

## 📋 Sobre o Projeto

O **Ambassador Contract** é um sistema de rastreamento de presença baseado em sessões com hashes criptográficos. Permite que administradores gerenciem sessões de eventos e usuários registrem sua presença de forma segura e verificável.

### Funcionalidades Principais

- ✅ **Gerenciamento de Sessões**: Administradores podem criar sessões com hashes únicos
- 👥 **Registro de Presença**: Usuários verificam presença fornecendo o hash correto da sessão
- 📝 **Perfis de Usuário**: Sistema de apelidos para identificação personalizada
- 🔍 **Consultas em Lote**: Verificação de presença de múltiplos usuários simultaneamente
- ⏰ **TTL Automático**: Gestão de tempo de vida de dados (7, 30 e 90 dias)
- 🔐 **Autorização**: Todas operações requerem autenticação apropriada

## 🏗️ Estrutura do Projeto

```text
.
├── contracts
│   └── ambassador-contract
│       ├── src
│       │   ├── lib.rs        # Implementação do contrato
│       │   └── test.rs       # Testes unitários
│       ├── Cargo.toml
│       └── Makefile          # Scripts de build e deploy
├── Cargo.toml                # Workspace configuration
├── WARP.md                   # Documentação para Warp AI
└── README.md
```

## 🚀 Início Rápido

### Pré-requisitos

- Rust 1.91.0 ou superior
- Stellar CLI 23.1.4 ou superior
- Target `wasm32v1-none` do Rust

```bash
# Instalar o target WASM
rustup target add wasm32v1-none

# Instalar Stellar CLI (se necessário)
cargo install --locked stellar-cli --features opt
```

### Build e Testes

```bash
# Navegar para o diretório do contrato
cd contracts/ambassador-contract

# Compilar o contrato
stellar contract build
# ou
make build

# Executar testes
cargo test
# ou
make test

# Formatar código
make fmt
```

## 📦 Deploy

### Deploy Completo (Testnet)

```bash
cd contracts/ambassador-contract
make deploy-testnet
```

Ou execute os passos manualmente:

```bash
# 1. Compilar para WASM
make build

# 2. Otimizar o WASM
make optimize

# 3. Instalar na rede
make install-testnet

# 4. Deploy do contrato
make deploy-only-testnet

# 5. Inicializar com endereço admin
make initialize-testnet ADMIN_ADDRESS=GABC...XYZ
```

### Variáveis de Ambiente

Crie um arquivo `.env` ou exporte:

```bash
export STELLAR_NETWORK=testnet
export STELLAR_SOURCE=alice  # ou seu identity
export ADMIN_ADDRESS=GABC...XYZ
```

## 📚 API do Contrato

### Funções Admin

#### `initialize(admin: Address)`
Inicializa o contrato com um administrador.

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source admin \
  -- initialize \
  --admin GABC...XYZ
```

#### `set_hash(new_hash: BytesN<32>)`
Cria uma nova sessão com um hash.

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source admin \
  -- set_hash \
  --new_hash 0123456789abcdef...
```

#### `transfer_admin(new_admin: Address)`
Transfere privilégios de admin para outro endereço.

### Funções de Usuário

#### `register(user: Address, submitted_hash: BytesN<32>)`
Registra presença do usuário na sessão atual.

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source user \
  -- register \
  --user GUSER...123 \
  --submitted_hash 0123456789abcdef...
```

#### `set_profile(user: Address, nickname: String)`
Define apelido do usuário (3-32 caracteres).

```bash
stellar contract invoke \
  --id CONTRACT_ID \
  --source user \
  -- set_profile \
  --user GUSER...123 \
  --nickname "Embaixador"
```

### Funções de Consulta (View)

#### `get_profile(user: Address) -> Option<UserProfile>`
Retorna perfil do usuário.

#### `check_presence(user: Address) -> bool`
Verifica se usuário está presente na sessão atual.

#### `check_batch(users: Vec<Address>) -> Vec<bool>`
Verifica presença de múltiplos usuários.

#### `get_admin() -> Address`
Retorna endereço do administrador atual.

#### `get_session() -> Option<BytesN<32>>`
Retorna hash da sessão ativa.

## 🔒 Modelo de Armazenamento

| Tipo | Storage | TTL | Descrição |
|------|---------|-----|----------|
| `Admin` | Instance | 30 dias | Endereço do administrador |
| `ActiveHash` | Persistent | 30 dias | Hash da sessão atual |
| `Presence(hash, user)` | Persistent | 30 dias | Registro de presença por sessão |
| `UserProfile(user)` | Persistent | 90 dias | Apelido e data de registro |

## 🧪 Testes

O contrato inclui testes unitários em `src/test.rs`. Para executar:

```bash
cd contracts/ambassador-contract
cargo test -- --nocapture
```

## 🛠️ Desenvolvimento

### Adicionar Novo Contrato

```bash
# Criar novo contrato no workspace
stellar contract init contracts/novo-contrato

# O Cargo.toml do workspace já está configurado para incluir contracts/*
```

### Profiles de Build

- **release**: Otimizado para produção (opt-level="z", LTO, strip symbols)
- **release-with-logs**: Release com assertions habilitadas para debug

## 📖 Recursos

- [Documentação Soroban](https://soroban.stellar.org/docs)
- [Stellar CLI Reference](https://developers.stellar.org/docs/tools/developer-tools/cli)
- [Soroban SDK Docs](https://docs.rs/soroban-sdk/latest/soroban_sdk/)

## 📄 Licença

Este projeto é fornecido como está, para fins educacionais e de desenvolvimento.

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Faça fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/NovaFuncionalidade`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/NovaFuncionalidade`)
5. Abra um Pull Request

---

**Nota**: Este é um projeto Soroban para a blockchain Stellar. Certifique-se de testar em testnet antes de fazer deploy em mainnet.
