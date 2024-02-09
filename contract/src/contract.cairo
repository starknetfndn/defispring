use starknet::ContractAddress;

#[starknet::interface]
pub trait IDistributor<TContractState> {
    fn claim(
        ref self: TContractState,
        claimee: ContractAddress,
        protocol: u8,
        amount: u128,
        proof: Span::<felt252>
    );

    fn add_root(ref self: TContractState, protocol: u8, new_root: felt252);

    fn get_root_for(
        self: @TContractState,
        claimee: ContractAddress,
        protocol: u8,
        amount: u128,
        proof: Span::<felt252>
    ) -> felt252;

    fn amount_already_claimed(
        self: @TContractState, claimee: ContractAddress, protocol: u8
    ) -> u128;

    fn roots_for_protocol(self: @TContractState, protocol: u8) -> Span<felt252>;
}

#[starknet::contract]
mod Distributor {
    use openzeppelin::access::ownable::ownable::OwnableComponent::InternalTrait;
use distributor::erc20::IERC20DispatcherTrait;
    use core::traits::TryInto;
    use distributor::contract::IDistributor;
    use starknet::ContractAddress;
    use core::array::{ArrayTrait, SpanTrait};
    use distributor::merkle_tree::{
        Hasher, MerkleTree, pedersen::PedersenHasherImpl, MerkleTreeTrait
    };
    use core::hash::LegacyHash;
    use openzeppelin::access::ownable::ownable::OwnableComponent;

    const STRK_ADDRESS: felt252 = 0x1234;

    #[storage]
    struct Storage {
        airdrop_claimed: LegacyMap::<(u8, ContractAddress), u128>,
        merkle_roots_per_protocol: LegacyMap::<(u8, u64), felt252>, // (protocol, round)
        #[substorage(v0)]
        ownable: OwnableComponent::Storage,
    }

    #[constructor]
    fn constructor(
        ref self: ContractState, owner: ContractAddress
    ) {
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
        protocol: u8,
        amount: u128
    }

    component!(path: OwnableComponent, storage: ownable, event: OwnableEvent);

    fn get_first_free_slot(self: @ContractState, protocol: u8) -> u64 {
        let mut i = 0;
        let mut root = self.merkle_roots_per_protocol.read((protocol, i));

        while root != 0 {
            i += 1;
            root = self.merkle_roots_per_protocol.read((protocol, i));
        };
        i
    }

    #[abi(embed_v0)]
    impl Distributor of super::IDistributor<ContractState> {
        fn claim(
            ref self: ContractState,
            claimee: ContractAddress,
            protocol: u8,
            amount: u128,
            proof: Span::<felt252>
        ) {
            let root = self.get_root_for(claimee, protocol, amount, proof);

            let roots = self.roots_for_protocol(protocol);
            let mut i = 0;
            loop {
                if (*roots.at(i) == root) {
                    let token = distributor::erc20::IERC20Dispatcher {
                        contract_address: STRK_ADDRESS.try_into().unwrap()
                    };
                    let left_to_claim = amount - self.airdrop_claimed.read((protocol, claimee));
                    token.transfer(claimee, u256 { high: 0, low: left_to_claim });
                    self.airdrop_claimed.write((protocol, claimee), amount);
                    self.emit(Claimed { claimee, protocol, amount });
                    break;
                }
                assert(i < roots.len(), 'INVALID PROOF');
                i += 1;
            };
        }

        fn get_root_for(
            self: @ContractState,
            claimee: ContractAddress,
            protocol: u8,
            amount: u128,
            proof: Span::<felt252>
        ) -> felt252 {
            let mut merkle_tree: MerkleTree<Hasher> = MerkleTreeTrait::new();

            let leaf = LegacyHash::hash(claimee.into(), amount);
            merkle_tree.compute_root(leaf, proof)
        }

        fn add_root(ref self: ContractState, protocol: u8, new_root: felt252) {
            self.ownable.assert_only_owner();
            let slot = get_first_free_slot(@self, protocol);
            self.merkle_roots_per_protocol.write((protocol, slot), new_root);
        }

        fn amount_already_claimed(
            self: @ContractState, claimee: ContractAddress, protocol: u8
        ) -> u128 {
            self.airdrop_claimed.read((protocol, claimee))
        }

        fn roots_for_protocol(self: @ContractState, protocol: u8) -> Span<felt252> {
            let mut res: Array<felt252> = ArrayTrait::new();
            let mut i: u64 = 0;
            loop {
                let curr_root: felt252 = self.merkle_roots_per_protocol.read((protocol, i));
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
