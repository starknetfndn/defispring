use starknet::ContractAddress;

// TODO: Ownership transfer? Upgrades?

#[starknet::interface]
pub trait IDistributor<TContractState> {
    fn claim(
        ref self: TContractState, claimee: ContractAddress, amount: u128, proof: Span::<felt252>
    );

    fn add_root(ref self: TContractState, new_root: felt252);

    fn get_root_for(
        self: @TContractState, claimee: ContractAddress, amount: u128, proof: Span::<felt252>
    ) -> felt252;

    fn amount_already_claimed(self: @TContractState, claimee: ContractAddress) -> u128;

    fn roots(self: @TContractState,) -> Span<felt252>;
}

#[starknet::contract]
mod Distributor {
    use openzeppelin::access::ownable::ownable::OwnableComponent::InternalTrait;
    use openzeppelin::token::erc20::interface::{IERC20Dispatcher, IERC20DispatcherTrait};
    use core::traits::TryInto;
    use distributor::contract::IDistributor;
    use starknet::ContractAddress;
    use core::array::{ArrayTrait, SpanTrait};
    use alexandria_merkle_tree::merkle_tree::{
        Hasher, MerkleTree, pedersen::PedersenHasherImpl, MerkleTreeTrait
    };
    use core::hash::LegacyHash;
    use openzeppelin::access::ownable::ownable::OwnableComponent;

    const STRK_ADDRESS: felt252 =
        0x04718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d; // Sepolia STRK. When it's deployed on mainnet, TODO update.

    #[storage]
    struct Storage {
        airdrop_claimed: LegacyMap::<ContractAddress, u128>,
        merkle_roots: LegacyMap::<u64, felt252>, // (round -> root)
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    #[constructor]
    fn constructor(ref self: ContractState, owner: ContractAddress) {
        self.ownable.initializer(owner);
    }

    #[derive(Drop, starknet::Event)]
    #[event]
    enum Event {
        Claimed: Claimed,
        OwnableEvent: OwnableComponent::Event
    }

    #[derive(Drop, starknet::Event)]
    struct Claimed {
        claimee: ContractAddress,
        amount: u128
    }

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    fn get_first_free_slot(self: @ContractState) -> u64 {
        let mut i = 0;
        let mut root = self.merkle_roots.read(i);

        while root != 0 {
            i += 1;
            root = self.merkle_roots.read(i);
        };
        i
    }

    #[abi(embed_v0)]
    impl Distributor of super::IDistributor<ContractState> {
        fn claim(
            ref self: ContractState, claimee: ContractAddress, amount: u128, proof: Span::<felt252>
        ) {
            let root = self.get_root_for(claimee, amount, proof);

            let roots = self.roots();
            let mut i = 0;
            loop {
                if (*roots.at(i) == root) {
                    let token = IERC20Dispatcher {
                        contract_address: STRK_ADDRESS.try_into().unwrap()
                    };
                    let left_to_claim = amount - self.airdrop_claimed.read(claimee);
                    token.transfer(claimee, u256 { high: 0, low: left_to_claim });
                    self.airdrop_claimed.write(claimee, amount);
                    self.emit(Claimed { claimee, amount });
                    break;
                }
                i += 1;
                assert(i < roots.len(), 'INVALID PROOF');
            };
        }

        fn get_root_for(
            self: @ContractState, claimee: ContractAddress, amount: u128, proof: Span::<felt252>
        ) -> felt252 {
            let mut merkle_tree: MerkleTree<Hasher> = MerkleTreeTrait::new();

            let leaf = LegacyHash::hash(claimee.into(), amount);
            merkle_tree.compute_root(leaf, proof)
        }

        fn add_root(ref self: ContractState, new_root: felt252) {
            self.ownable.assert_only_owner();
            let slot = get_first_free_slot(@self);
            self.merkle_roots.write(slot, new_root);
        }

        fn amount_already_claimed(self: @ContractState, claimee: ContractAddress) -> u128 {
            self.airdrop_claimed.read(claimee)
        }

        fn roots(self: @ContractState) -> Span<felt252> {
            let mut res: Array<felt252> = ArrayTrait::new();
            let mut i: u64 = 0;
            loop {
                let curr_root: felt252 = self.merkle_roots.read(i);
                i += 1;
                if (curr_root == 0) {
                    break;
                }
                res.append(curr_root);
            };
            res.span()
        }
    }
}
