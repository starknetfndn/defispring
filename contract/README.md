# Smart contract documentation

## TODO

- functionality
- usage
- deployment instructions
- ...

## Notes

### Enough liquidity for the allocation

The smart contract assumes there is enough liqudity do the entire allocation. The smart contract also does not have any information or a way to check the total amount that is yet to be distributed, it has only the already distributed amounts and the roots of the merkle trees.

It is assumed that the smart contract owner will take care of this manually.

### KYC

In case protocols have to do KYC they can be updating (adding new users for allocations) the roots more often that it would be needed otherwise. For example on a daily basis.
