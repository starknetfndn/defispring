use core::hash::HashStateExTrait;
use core::{ArrayTrait, SpanTrait};
use core::debug::PrintTrait;
use distributor::contract::{Distributor, IDistributorDispatcher, IDistributorDispatcherTrait};
use Distributor::STRK_ADDRESS;
use openzeppelin::token::erc20::interface::{IERC20Dispatcher, IERC20DispatcherTrait};
use snforge_std::{ContractClassTrait, declare, start_prank, CheatTarget};
use starknet::{ContractAddress, deploy_syscall};

const ADMIN_ADDR: felt252 = 0x42;
const CLAIMEE_1: felt252 = 0x13;
const CLAIMEE_2: felt252 = 0x14;
const CLAIMEE_3: felt252 = 0x21;

fn deploy() -> IDistributorDispatcher {
    let mut calldata = ArrayTrait::new();
    calldata.append(ADMIN_ADDR);

    let contract = declare('Distributor');
    let address = contract.deploy(@calldata).expect('unable to deploy distributor');

    IDistributorDispatcher { contract_address: address }
}

fn deploy_token(recipient: ContractAddress) -> IERC20Dispatcher {
    let mut calldata = ArrayTrait::new();
    calldata.append(1000000000000000000);
    calldata.append(0);
    calldata.append(recipient.into());
    let contract = declare('MyToken');
    let address = contract
        .deploy_at(@calldata, STRK_ADDRESS.try_into().unwrap())
        .expect('unable to deploy mockstrk');

    IERC20Dispatcher { contract_address: address }
}


#[test]
fn test_single_claims_multiple_roots() {
    let contract = deploy();
    let tok = deploy_token(contract.contract_address);
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());

    contract.add_root(0xe5c5a70b996a566aa28559817bac9a79a6575090abaa9509f606e1b25dd98); // decoy
    contract.add_root(0xf7c8d3f309262572ad35df8ff6c33f24d8114c60eac3bc27bf42382ca82faf);
    contract.add_root(0x2f582855ca3f9bb074b939b1670554bd01334b0bc9fe95ed7577295db1086b); // decoy
    contract.add_root(0x3af4d227c0978ff30099df450f64676ef25f9255d4fa36f900be2aed17f332d); // claimee 2 only

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_1.try_into().unwrap()
    );

    let proof_1 = array![
        0x2a18afb0550a011d54ca3940648e59894c06e4c3d0a611256c0b575bd528b3b
    ];

    contract.claim(0x88, proof_1.span());
    assert(tok.balance_of(CLAIMEE_1.try_into().unwrap()) == 0x88, 'wrong bal claimee 1');

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_2.try_into().unwrap()
    );

    let proof_2 = array![
        0x7fa669b18489a1632df0de6e4d2b58558457b10fbcdeae8975e7e4d8d2e15db
    ];

    contract.claim(0x89, proof_2.span());
    assert(tok.balance_of(CLAIMEE_2.try_into().unwrap()) == 0x89, 'wrong bal claimee 2');

    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0x5915822da3096dbf676bd0b5b2c2b27638535a85d568e0e7f282c46c1fb3577); // decoy

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_3.try_into().unwrap()
    );

    let proof_3 = array![
        0x1373596138a034686261d487a628b90e33c38391196fe57a4378cfe5a4af3fa
    ];

    contract.claim(0xd5, proof_3.span());
    assert(tok.balance_of(CLAIMEE_3.try_into().unwrap()) == 0xd5, 'wrong bal claimee 3');
    // CLAIMEE_3 Trying to claim the same allocation twice
    contract.claim(0xd5, proof_3.span());
    assert(tok.balance_of(CLAIMEE_3.try_into().unwrap()) == 0xd5, 'wrong bal claimee 3');

    // Checking that CLAIMEE_3 claiming did not effect first two claimees.
    assert(tok.balance_of(CLAIMEE_1.try_into().unwrap()) == 0x88, 'wrong bal claimee 1');
    assert(tok.balance_of(CLAIMEE_2.try_into().unwrap()) == 0x89, 'wrong bal claimee 2');
}

#[test]
#[should_panic(expected: ('INVALID PROOF',))]
fn test_claim_invalid_proof() {
    let contract = deploy();
    deploy_token(contract.contract_address);
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0xf7c8d3f309262572ad35df8ff6c33f24d8114c60eac3bc27bf42382ca82faf);

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_1.try_into().unwrap()
    );
    let proof = array![
        0x2a18afb0550a011d54ca3940648e59894c06e4c3d0a611256c0b575bd528b3b,
        0x1
    ];
    contract.claim(0x88, proof.span());
}

// Wrong person claims with proof that is correct for someone else.
#[test]
#[should_panic(expected: ('INVALID PROOF',))]
fn test_claim_wrong_claimee() {
    let contract = deploy();
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe);

    // This root is for CLAIMEE_1. Testing if CLAIMEE_2 claims it.
    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_2.try_into().unwrap()
    );

    let proof_1 = array![
        0x2a18afb0550a011d54ca3940648e59894c06e4c3d0a611256c0b575bd528b3b
    ];

    contract.claim(0x88, proof_1.span());
}

#[test]
#[should_panic(expected: ('INVALID PROOF',))]
fn test_claim_wrong_amount() {
    let contract = deploy();
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe);

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_1.try_into().unwrap()
    );

    let proof_1 = array![
        0x2a18afb0550a011d54ca3940648e59894c06e4c3d0a611256c0b575bd528b3b
    ];

    // correct amount is 0x88.
    contract.claim(0x89, proof_1.span());
}

#[test]
fn test_compute_root() {
    let contract = deploy();
    let proof = array![
        0x2a18afb0550a011d54ca3940648e59894c06e4c3d0a611256c0b575bd528b3b
    ];
    let root = contract.get_root_for(CLAIMEE_1.try_into().unwrap(), 0x88, proof.span());
    assert(
        root == 0xf7c8d3f309262572ad35df8ff6c33f24d8114c60eac3bc27bf42382ca82faf,
        'roots dont match'
    );
    let root = contract.get_root_for(CLAIMEE_1.try_into().unwrap(), 0x0, proof.span());
    assert(root != 0xf7c8d3f309262572ad35df8ff6c33f24d8114c60eac3bc27bf42382ca82faf, 'wrong root');
}
