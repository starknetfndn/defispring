# Example frontend

This is an example frontend that can be used to claim allocations.

## Overview

The frontend is implemented with NextJS (React). It has the bare minimum functionality required to showcase the functionality.

Do not use this functionality directly in your own projects. Instead, adjust it to your own project.

## Functionality

This frontend has the following functionality:

1. Connect your Starknet wallet
1. Check how many tokens you have been allocated
1. Check how many tokens you have already claimed
1. Prepare to claim your tokens (retrieves some needed metadata)
1. Claim your tokens. Requires that you first execute the preparation phase

## Trying it out

All of the functionality is only for demonstration purposes. However, if you want to test it out yourself, you need to do at least the following changes to the code:

1. Change the BASE_BACKEND_URL in page.tsx to point to your backend. Or leave as is, if you are testing locally (and running the backend in that address)
1. Change the CONTRACT_ADDRESS to point to your deployed contract
1. Change the used/supported networks in starknet-provider.tsx
1. Make sure the used ABI is up to date (abi.json in the _app_ folder)

## Implementation details

When you implement this yourself, make sure of at least the following nasty details:

1. All address are zero-padded at the beginning

### Installation

This is a regular NextJS project with npm. Therefore, installation steps are:

1. Run `npm install`
1. Run `npm run build` to build some metadata needed by NextJS
1. Run `npm run dev` to run locally
