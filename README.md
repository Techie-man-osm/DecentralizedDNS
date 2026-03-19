# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
# 🌐 Decentralized DNS on Stellar (Soroban)

## 📌 Project Description

This project is a decentralized Domain Name System (DNS) built using Soroban smart contracts on the Stellar blockchain. It removes reliance on centralized authorities and enables users to manage domain records securely on-chain.

## ⚙️ What it does

* Allows users to register domain names
* Maps domains to IP addresses
* Enables domain resolution (domain → IP)
* Allows only the owner to update domain records

## ✨ Features

* 🔐 Ownership-based access control
* 🌍 Fully decentralized system
* ⚡ Fast domain resolution
* 🧱 Secure on-chain storage
* 🔄 Easy record updates

## 🚀 Deployed Smart link :
https://stellar.expert/explorer/testnet/tx/d4901390fd3110262edebf06cd2d91f381e1db13b6dbd33896f5deca62948d4f
<img width="1920" height="1080" alt="Screenshot 2026-03-19 141726" src="https://github.com/user-attachments/assets/93bb9932-8fc0-44b4-b8f9-b05f9244e41b" />


