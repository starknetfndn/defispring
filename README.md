# DeFi incentives allocation manager

This project is used to enable allocations in the Starknet ecosystem for given addresses. The project is utilized for incentivizing DeFi protocol usage.

The project contains a backend that generates all the needed data for the allocations, a Cairo contract for distributing the allocations and a frontend to demonstrate the functionality.

## Usage for different parties

Each Starknet DeFi project that wants to utilize these incentivized allocations is expected to:

1. Deploy the backend to their own cloud provider
1. Utilize the backend to generate data for allocations
1. Deploy the Cairo contract and update its data
1. Add a button to their frontend for users to claim their allocations, utilizing the Cairo contract

## Backend

The backend is documented at <a href='backend/README.md'>the backend folder</a>.

## Cairo Smart Contract

The Cairo contract is documented at <a href='contract/README.md'>the contract folder</a>.

## Frontend

The frontend is documented at <a href='frontend/README.md'>the frontend folder</a>.

## Testnet deployment

Smart contract is deployed on Sepolia at address: [0x06781eddde09e243eb4280ec8e6a9ba6aced153c4da1ddd059adf9ea61e51526](https://sepolia.voyager.online/contract/0x06781eddde09e243eb4280ec8e6a9ba6aced153c4da1ddd059adf9ea61e51526#readContract)

Backend (for the testnet) is deployed at: [35.195.237.203:8080](http://35.195.237.203:8080/swagger-ui/)

Frontend (for the testnet) is deployed at: [bluh-bleh-bloh-dummy.vercel.app](https://bluh-bleh-bloh-dummy.vercel.app/)

The addresses are not-official seeming on purpose to not give anyone a sense that they are for anything else than testing purposes.
