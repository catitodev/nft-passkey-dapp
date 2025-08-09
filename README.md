# 🎨 NFT PassKey DApp

Uma DApp (Aplicação Descentralizada) para criação e gerenciamento de NFTs usando autenticação **PassKey/WebAuthn** em Rust.

## ✨ Funcionalidades

- 🔐 **Autenticação PassKey**: Login seguro sem senhas usando biometria/YubiKey
- 🎨 **Criação de NFTs**: Interface para criar coleções e mintar tokens
- 🏪 **Marketplace**: Sistema básico de compra/venda de NFTs
- 🔗 **Integração Blockchain**: Suporte para Ethereum/Polygon/Solana/Celo
- 📱 **Responsivo**: Interface moderna e mobile-friendly

## 🚀 Tecnologias

- **Backend**: Rust + Warp + WebAuthn-RS
- **Frontend**: HTML5 + JavaScript vanilla + CSS3
- **Blockchain**: Ethers.rs para interação com contratos
- **Autenticação**: WebAuthn/FIDO2 (PassKeys)
- **Banco**: SQLite

## 📦 Instalação

### Pré-requisitos
- Rust 1.75+
- Git
- Navegador com suporte a WebAuthn

### Executar Localmente

1. **Clone o repositório**:
   ```bash
   git clone https://github.com/catitodev/nft-passkey-dapp.git
   cd nft-passkey-dapp
   