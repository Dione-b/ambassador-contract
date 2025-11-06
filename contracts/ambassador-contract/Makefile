# Makefile para Ambassador Contract
# Variáveis de configuração
NETWORK ?= testnet
SOURCE ?= alice
ADMIN_ADDRESS ?= 

# Caminhos dos arquivos
WASM_FILE = ../../target/wasm32v1-none/release/ambassador_contract.wasm
OPTIMIZED_WASM = ../../target/wasm32v1-none/release/ambassador_contract.optimized.wasm

# Arquivo para armazenar variáveis entre etapas
DEPLOY_VARS = .deploy_vars

.PHONY: default all test build fmt clean optimize install deploy initialize help

default: build

all: test

## Comandos de Desenvolvimento

test: build
	@echo "⚡ Executando testes..."
	cargo test

build:
	@echo "🔨 Compilando contrato..."
	stellar contract build
	@ls -l $(WASM_FILE)

fmt:
	@echo "✨ Formatando código..."
	cargo fmt --all

clean:
	@echo "🧹 Limpando arquivos de build..."
	cargo clean
	rm -f $(DEPLOY_VARS)

## Comandos de Deploy

optimize: build
	@echo "🚀 Otimizando WASM..."
	stellar contract optimize --wasm $(WASM_FILE)
	@ls -lh $(OPTIMIZED_WASM)

install: optimize
	@echo "📦 Instalando contrato na rede $(NETWORK)..."
	@WASM_HASH=$$(stellar contract install \
		--wasm $(OPTIMIZED_WASM) \
		--network $(NETWORK) \
		--source $(SOURCE)); \
	echo "WASM_HASH=$$WASM_HASH" > $(DEPLOY_VARS); \
	echo "✅ WASM Hash: $$WASM_HASH"

install-testnet:
	@$(MAKE) install NETWORK=testnet

install-futurenet:
	@$(MAKE) install NETWORK=futurenet

deploy-only:
	@echo "🚀 Fazendo deploy do contrato na rede $(NETWORK)..."
	@if [ ! -f $(DEPLOY_VARS) ]; then \
		echo "❌ Erro: Execute 'make install' primeiro"; \
		exit 1; \
	fi
	@. $(DEPLOY_VARS); \
	CONTRACT_ID=$$(stellar contract deploy \
		--wasm-hash $$WASM_HASH \
		--network $(NETWORK) \
		--source $(SOURCE)); \
	echo "CONTRACT_ID=$$CONTRACT_ID" >> $(DEPLOY_VARS); \
	echo "✅ Contract ID: $$CONTRACT_ID"

deploy-only-testnet:
	@$(MAKE) deploy-only NETWORK=testnet

deploy-only-futurenet:
	@$(MAKE) deploy-only NETWORK=futurenet

initialize:
	@echo "⚙️ Inicializando contrato..."
	@if [ ! -f $(DEPLOY_VARS) ]; then \
		echo "❌ Erro: Execute 'make deploy-only' primeiro"; \
		exit 1; \
	fi
	@if [ -z "$(ADMIN_ADDRESS)" ]; then \
		echo "❌ Erro: Defina ADMIN_ADDRESS"; \
		echo "Uso: make initialize ADMIN_ADDRESS=GABC...XYZ"; \
		exit 1; \
	fi
	@. $(DEPLOY_VARS); \
	stellar contract invoke \
		--id $$CONTRACT_ID \
		--network $(NETWORK) \
		--source $(SOURCE) \
		-- \
		initialize \
		--admin $(ADMIN_ADDRESS); \
	echo "✅ Contrato inicializado com admin: $(ADMIN_ADDRESS)"

initialize-testnet:
	@$(MAKE) initialize NETWORK=testnet

initialize-futurenet:
	@$(MAKE) initialize NETWORK=futurenet

# Deploy completo (instalar + deploy + inicializar)
deploy: install deploy-only
	@echo "✅ Deploy completo! Execute 'make initialize ADMIN_ADDRESS=...' para inicializar"

deploy-testnet:
	@$(MAKE) deploy NETWORK=testnet

deploy-futurenet:
	@$(MAKE) deploy NETWORK=futurenet

# Deploy full com inicialização
deploy-full: install deploy-only initialize
	@echo "🎉 Deploy e inicialização completos!"

deploy-full-testnet:
	@if [ -z "$(ADMIN_ADDRESS)" ]; then \
		echo "❌ Erro: Defina ADMIN_ADDRESS"; \
		echo "Uso: make deploy-full-testnet ADMIN_ADDRESS=GABC...XYZ"; \
		exit 1; \
	fi
	@$(MAKE) deploy-full NETWORK=testnet

## Comandos de Utilidade

show-vars:
	@if [ -f $(DEPLOY_VARS) ]; then \
		echo "📋 Variáveis de deploy:"; \
		cat $(DEPLOY_VARS); \
	else \
		echo "❌ Nenhum deploy encontrado"; \
	fi

help:
	@echo "Ambassador Contract - Makefile"
	@echo ""
	@echo "Comandos disponíveis:"
	@echo ""
	@echo "  Desenvolvimento:"
	@echo "    make build              - Compila o contrato"
	@echo "    make test               - Executa testes"
	@echo "    make fmt                - Formata o código"
	@echo "    make clean              - Limpa arquivos de build"
	@echo ""
	@echo "  Deploy (passo a passo):"
	@echo "    make optimize           - Otimiza o WASM"
	@echo "    make install-testnet    - Instala na testnet"
	@echo "    make deploy-only-testnet - Faz deploy na testnet"
	@echo "    make initialize-testnet ADMIN_ADDRESS=G... - Inicializa"
	@echo ""
	@echo "  Deploy (completo):"
	@echo "    make deploy-testnet     - Instala e faz deploy (sem inicializar)"
	@echo "    make deploy-full-testnet ADMIN_ADDRESS=G... - Deploy completo"
	@echo ""
	@echo "  Utilidade:"
	@echo "    make show-vars          - Mostra variáveis de deploy"
	@echo "    make help               - Mostra esta ajuda"
	@echo ""
	@echo "Variáveis:"
	@echo "  NETWORK=testnet|futurenet|mainnet (padrão: testnet)"
	@echo "  SOURCE=alice|identity_name (padrão: alice)"
	@echo "  ADMIN_ADDRESS=GABC...XYZ (obrigatório para initialize)"
	@echo ""
	@echo "Exemplo:"
	@echo "  make deploy-full-testnet SOURCE=minha-identity ADMIN_ADDRESS=GABC..."
