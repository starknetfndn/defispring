use core::result::ResultTrait;
use distributor::contract::{Distributor, IDistributorDispatcher, IDistributorDispatcherTrait};
use Distributor::STRK_ADDRESS;
use starknet::{ContractAddress, deploy_syscall};
use core::{ArrayTrait, SpanTrait};
use core::debug::PrintTrait;
use openzeppelin::token::erc20::interface::{IERC20Dispatcher, IERC20DispatcherTrait};
use snforge_std::{ContractClassTrait, declare, start_prank, CheatTarget};


const ADMIN_ADDR: felt252 = 0x42;
const CLAIMEE_1: felt252 = 0x13;

fn deploy() -> IDistributorDispatcher {
    let mut calldata = ArrayTrait::new();
    calldata.append(ADMIN_ADDR);
    //let (address, _) = deploy_syscall(
    //    Distributor::TEST_CLASS_HASH.try_into().unwrap(), 0, calldata.span(), true // todo check all params and what they mean
    //).expect('contract deploy failed');

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
fn test_claim_one_root() {
    let contract = deploy();
    deploy_token(contract.contract_address);
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe);

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_1.try_into().unwrap()
    );
    let proof = array![
        0x2f582855ca3f9bb074b939b1670554bd01334b0bc9fe95ed7577295db1086b,
        0xe5c5a70b996a566aa28559817bac9a79a6575090abaa9509f606e1b25dd98
    ];
    contract.claim(0x88, proof.span());
}

#[test]
#[should_panic(expected: ('INVALID PROOF',))]
fn test_claim_invalid_proof() {
    let contract = deploy();
    deploy_token(contract.contract_address);
    start_prank(CheatTarget::One(contract.contract_address), ADMIN_ADDR.try_into().unwrap());
    contract.add_root(0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe);

    start_prank(
        CheatTarget::One(contract.contract_address), CLAIMEE_1.try_into().unwrap()
    );
    let proof = array![
        0x2f582855ca3f9bb074b939b1670554bd01334b0bc9fe95ed7577295db1086b,
        0xe5c5a70b996a566aa28559817bac9a79a6575090abaa9509f606e1b25dd98,
        0x1
    ];
    contract.claim(0x88, proof.span());
}

#[test]
fn test_compute_root() {
    let contract = deploy();
    let proof = array![
        0x2f582855ca3f9bb074b939b1670554bd01334b0bc9fe95ed7577295db1086b,
        0xe5c5a70b996a566aa28559817bac9a79a6575090abaa9509f606e1b25dd98
    ];
    let root = contract.get_root_for(CLAIMEE_1.try_into().unwrap(), 0x88, proof.span());
    assert(
        root == 0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe,
        'roots dont match'
    );
    let root = contract.get_root_for(CLAIMEE_1.try_into().unwrap(), 0x0, proof.span());
    assert(root != 0x45aa6b933e7b76e85c77fc12b2cc58c22ba87b76fb7595bd315fb3ede730dfe, 'wrong root');
}
