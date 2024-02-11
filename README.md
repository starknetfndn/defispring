# DeFi incentives airdrop manager

This project is used to enable airdrops in the Starknet ecosystem for given addresses. The project is utilized for incentivizing DeFi protocol usage.

The project contains a backend that generates all the needed data for the airdrops, a Cairo contract for distributing the airdrops and a frontend to demonstrate the functionalit.

## Usage for different parties

Each Starknet DeFi project that wants to utilize these incentivized airdrops is expected to:

1. Deploy the backend to their own cloud provider
1. Utilize the backend to generate data for airdrops
1. Deploy the Cairo contract and update its data
1. Add a button to their frontend for users to claim their airdrops, utilizing the Cairo contract

## Backend

The backend is documented at <a href='backend/README.md'>the backend folder</a>.

## Cairo Smart Contract

The Cairo contract is documented at <a href='contract/README.md'>the contract folder</a>.

## Frontend

The frontend is documented at <a href='frontend/README.md'>the frontend folder</a>.
