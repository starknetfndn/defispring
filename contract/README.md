# DeFi Incentives Distribution Contract

## Deployment

You need snforge 0.17.0 and scarb 2.5.3 (with Cairo 2.5.3)

Run tests: `snforge test`
Build the contract: `scarb build`

After that you can deploy 'as usual' with starkli/sncast.

## Interacting with the contract

The contract is to be deployed by each protocol that distributes incentives separately.

In the constructor, you pass an account that cannot be changed later. It is recommended that this be e.g. a multisig.

When there is a new round of incentives, add a new root by invoking `add_root`.

During claiming, the merkle root is evaluated and compared to all previously added roots. If at least one matches, the funds are distributed and the `amount_already_claimed` is updated. The user always receives the amount requested in the claim (if the proof is correct; the amount is part of the proof and hence has to correspond to the value in the merkle tree) minus `amount_already_claimed`.

## Notes

### Enough liquidity for the allocation

The smart contract assumes there is enough liqudity do the entire allocation. The smart contract also does not have any information or a way to check the total amount that is yet to be distributed, it has only the already distributed amounts and the roots of the merkle trees.

It is assumed that the smart contract owner will take care of this manually.

### KYC

In case protocols have to do KYC they can be updating (adding new users for allocations) the roots more often that it would be needed otherwise. For example on a daily basis.

### Security of the owner account

It is assumed that the owner account is not a malicious account. It is assumed that the owner account will not try to extract capital from the smart contract by adding "malicious root". This is assumed that the StarkNet Foundation can take care of by a legal contract.

### Merkle root IDs

Merkle root IDs in the merkle_roots storage do not have to correspond to the incentive allocation "in the real world". This ID (number) is not used from the "outside".
