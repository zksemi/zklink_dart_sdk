use std::str::FromStr;
use std::time::Instant;
use anyhow::Result;
use serde_json::to_string;
use flutter_rust_bridge::frb;
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_signers::{eth_signer::PackedEthSignature, zklink_signer::PubKeyHash};
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::error::TypeError::{DecodeFromHexErr, InvalidBigIntStr};
use zklink_sdk_types::tx_builder::*;
use zklink_sdk_types::tx_type::change_pubkey::Create2Data;
use zklink_sdk_types::tx_type::contract::*;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::order_matching::{Order, OrderMatching};
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::{basic_types::BigUint, tx_type::change_pubkey::ChangePubKey};
use zklink_sdk_types::prelude::ChangePubKeyBuilder;
use zklink_sdk_interface::signer::{Signer, L1SignerType};

#[frb(sync)]
pub fn eth_signer(eth_private_key: String) -> Result<Signer> {
    Ok(Signer::new(&eth_private_key, L1SignerType::Eth)?)
}

#[frb(sync)]
pub fn starknet_signer(
    eth_private_key: String,
    starknet_chain_id: String,
    starknet_addr: String
) -> Result<Signer> {
    let signer_type = L1SignerType::Starknet {
        chain_id: starknet_chain_id,
        address: starknet_addr,
    };
    Ok(Signer::new(&eth_private_key, signer_type)?)
}

#[frb(sync)]
pub fn change_pub_key(
    chain_id: u8,
    account_id: u32,
    sub_account_id: u8,
    new_pubkey_hash: String,
    fee_token: u32,
    fee: String,
    nonce: u32,
    eth_signature: Option<String>,
    ts: Option<u32>,
) -> Result<ChangePubKey> {
    let ts = if let Some(time_stamp) = ts {
        time_stamp
    } else {
        Instant::now().elapsed().as_secs() as u32
    };
    let eth_signature = if let Some(s) = eth_signature {
        Some(PackedEthSignature::from_hex(&s)?)
    } else {
        None
    };
    Ok(ChangePubKeyBuilder {
        chain_id: chain_id.into(),
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        new_pubkey_hash: PubKeyHash::from_hex(&new_pubkey_hash)?,
        fee_token: fee_token.into(),
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        nonce: nonce.into(),
        eth_signature,
        timestamp: ts.into(),
    }.build())
}

#[frb(sync)]
pub fn get_pubkey(signer: Signer) -> String {
    signer.public_key().as_hex()
}

#[frb(sync)]
pub fn get_pubkey_hash(signer: Signer) -> String {
    signer.pubkey_hash().as_hex()
}

#[frb(sync)]
pub fn sign_change_pubkey_with_onchain(
    signer: Signer,
    tx: ChangePubKey,
) -> Result<String> {
    let sig = signer.sign_change_pubkey_with_onchain_auth_data(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn sign_change_pubkey_with_eth_ecdsa_auth(
    signer: Signer,
    tx: ChangePubKey,
) -> Result<String> {
    let sig = signer.sign_change_pubkey_with_eth_ecdsa_auth(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn sign_change_pubkey_with_create2data_auth(
    signer: Signer,
    tx: ChangePubKey,
    creator_address: String,
    salt_arg: String,
    code_hash: String,
) -> Result<String> {
    let create2_data = Create2Data {
        creator_address: ZkLinkAddress::from_hex(&creator_address)?,
        code_hash: H256::from_str(&code_hash)?,
        salt_arg: H256::from_str(&salt_arg)?,
    };
    let sig = signer.sign_change_pubkey_with_create2data_auth(tx, create2_data)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn transfer(
    account_id: u32,
    to_address: String,
    from_sub_account_id: u8,
    to_sub_account_id: u8,
    token: u32,
    fee: String,
    amount: String,
    nonce: u32,
    ts: Option<u32>,
) -> Result<Transfer> {
    let ts = if let Some(time_stamp) = ts {
        time_stamp
    } else {
        Instant::now().elapsed().as_secs() as u32
    };
    Ok(TransferBuilder {
        account_id: account_id.into(),
        to_address: ZkLinkAddress::from_hex(&to_address)?,
        from_sub_account_id: from_sub_account_id.into(),
        to_sub_account_id: to_sub_account_id.into(),
        token: token.into(),
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        nonce: nonce.into(),
        timestamp: ts.into(),
        amount: BigUint::from_str(&amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
    }.build())
}

#[frb(sync)]
pub fn sign_transfer(
    signer: Signer,
    tx: Transfer,
    token_symbol: String,
    chain_id: Option<String>,
    addr: Option<String>,
) -> Result<String> {
    let sig = signer.sign_transfer(tx, &token_symbol, chain_id, addr)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn withdraw(
    account_id: u32,
    sub_account_id: u8,
    to_chain_id: u8,
    to_address: String,
    l2_source_token: u32,
    l1_target_token: u32,
    amount: String,
    data_hash: Option<String>,
    fee: String,
    nonce: u32,
    withdraw_to_l1: bool,
    withdraw_fee_ratio: u16,
    ts: Option<u32>,
) -> Result<Withdraw> {
    let ts = if let Some(time_stamp) = ts {
        time_stamp
    } else {
        Instant::now().elapsed().as_secs() as u32
    };
    let data_hash = if let Some(data_hash) = data_hash {
        Some(H256::from_str(&data_hash).map_err(|e| DecodeFromHexErr(e.to_string()))?)
    } else {
        None
    };
    Ok(WithdrawBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        to_chain_id: to_chain_id.into(),
        to_address: ZkLinkAddress::from_hex(&to_address)?,
        l2_source_token: l2_source_token.into(),
        l1_target_token: l1_target_token.into(),
        amount: BigUint::from_str(&amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        data_hash,
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        nonce: nonce.into(),
        withdraw_to_l1,
        withdraw_fee_ratio,
        timestamp: ts.into(),
    }.build())
}

#[frb(sync)]
pub fn sign_withdraw(
    signer: Signer,
    tx: Withdraw,
    token_symbol: String,
    chain_id: Option<String>,
    addr: Option<String>,
) -> Result<String> {
    let sig = signer.sign_withdraw(tx, &token_symbol, chain_id, addr)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn forced_exit(
    to_chain_id: u8,
    initiator_account_id: u32,
    initiator_sub_account_id: u8,
    target_sub_account_id: u8,
    target: String,
    l2_source_token: u32,
    l1_target_token: u32,
    exit_amount: String,
    initiator_nonce: u32,
    withdraw_to_l1: bool,
    ts: Option<u32>,
) -> Result<ForcedExit> {
    let ts = if let Some(time_stamp) = ts {
        time_stamp
    } else {
        Instant::now().elapsed().as_secs() as u32
    };
    Ok(ForcedExitBuilder {
        to_chain_id: to_chain_id.into(),
        initiator_account_id: initiator_account_id.into(),
        initiator_sub_account_id: initiator_sub_account_id.into(),
        target: ZkLinkAddress::from_hex(&target)?,
        l2_source_token: l2_source_token.into(),
        timestamp: ts.into(),
        l1_target_token: l1_target_token.into(),
        initiator_nonce: initiator_nonce.into(),
        target_sub_account_id: target_sub_account_id.into(),
        withdraw_to_l1,
        exit_amount: BigUint::from_str(&exit_amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
    }.build())
}

#[frb(sync)]
pub fn sign_forced_exit(signer: Signer, tx: ForcedExit) -> Result<String> {
    let sig = signer.sign_forced_exit(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn contract_price(pair_id: u16, market_price: String) -> Result<ContractPrice> {
    Ok(ContractPrice {
        pair_id: pair_id.into(),
        market_price: BigUint::from_str(&market_price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
    })
}

#[frb(sync)]
pub fn spot_price(token_id: u32, price: String) -> Result<SpotPriceInfo> {
    Ok(SpotPriceInfo {
        token_id: token_id.into(),
        price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
    })
}

#[frb(sync)]
pub fn order(
    account_id: u32,
    sub_account_id: u8,
    slot_id: u32,
    nonce: u32,
    base_token_id: u32,
    quote_token_id: u32,
    amount: String,
    price: String,
    is_sell: bool,
    maker_fee_rate: u8,
    taker_fee_rate: u8,
    has_subsidy: bool,
) -> Result<Order> {
    Ok(Order {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        slot_id: slot_id.into(),
        nonce: nonce.into(),
        base_token_id: base_token_id.into(),
        quote_token_id: quote_token_id.into(),
        amount: BigUint::from_str(&amount).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        is_sell: is_sell as u8,
        fee_rates: [maker_fee_rate, taker_fee_rate],
        has_subsidy: has_subsidy as u8,
        signature: Default::default(),
    })
}

#[frb(sync)]
pub fn order_matching(
    account_id: u32,
    sub_account_id: u8,
    taker: Order,
    maker: Order,
    fee: String,
    fee_token: u32,
    contract_prices: Vec<ContractPrice>,
    margin_prices: Vec<SpotPriceInfo>,
    expect_base_amount: String,
    expect_quote_amount: String,
) -> Result<OrderMatching> {
    Ok(OrderMatchingBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        taker,
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee_token: fee_token.into(),
        expect_base_amount: BigUint::from_str(&expect_base_amount)
            .map_err(|e| InvalidBigIntStr(e.to_string()))?,
        maker,
        expect_quote_amount: BigUint::from_str(&expect_quote_amount)
            .map_err(|e| InvalidBigIntStr(e.to_string()))?,
        contract_prices,
        margin_prices,
    }.build())
}

#[frb(sync)]
pub fn create_signed_order(signer: Signer, order: Order) -> Result<Order> {
    Ok(signer.create_signed_order(&order)?)
}

#[frb(sync)]
pub fn sign_order_matching(signer: Signer, tx: OrderMatching) -> Result<String> {
    let sig = signer.sign_order_matching(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn contract(
    account_id: u32,
    sub_account_id: u8,
    slot_id: u32,
    nonce: u32,
    pair_id: u16,
    size: String,
    price: String,
    direction: bool,
    maker_fee_rate: u8,
    taker_fee_rate: u8,
    has_subsidy: bool,
) -> Result<Contract> {
    Ok(ContractBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        slot_id: slot_id.into(),
        nonce: nonce.into(),
        pair_id: pair_id.into(),
        size: BigUint::from_str(&size).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        direction,
        maker_fee_rate,
        taker_fee_rate,
        has_subsidy,
    }.build())
}

#[frb(sync)]
pub fn contract_matching(
    account_id: u32,
    sub_account_id: u8,
    taker: Contract,
    maker: Vec<Contract>,
    fee: String,
    fee_token: u16,
    contract_prices: Vec<ContractPrice>,
    margin_prices: Vec<SpotPriceInfo>,
) -> Result<ContractMatching> {
    Ok(ContractMatchingBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        taker,
        maker,
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee_token: fee_token.into(),
        contract_prices,
        margin_prices,
    }.build())
}

#[frb(sync)]
pub fn create_signed_contract(signer: Signer, contract: Contract) -> Result<Contract> {
    Ok(signer.create_signed_contract(&contract)?)
}

#[frb(sync)]
pub fn sign_contract_matching(signer: Signer, tx: ContractMatching) -> Result<String> {
    let sig = signer.sign_contract_matching(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn auto_eleveraging(
    account_id: u32,
    sub_account_id: u8,
    sub_account_nonce: u32,
    contract_prices: Vec<ContractPrice>,
    margin_prices: Vec<SpotPriceInfo>,
    adl_account_id: u32,
    pair_id: u16,
    adl_size: String,
    adl_price: String,
    fee: String,
    fee_token: u16,
) -> Result<AutoDeleveraging> {
    Ok(AutoDeleveragingBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        sub_account_nonce: sub_account_nonce.into(),
        contract_prices,
        margin_prices,
        adl_account_id: adl_account_id.into(),
        pair_id: pair_id.into(),
        adl_size: BigUint::from_str(&adl_size).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        adl_price: BigUint::from_str(&adl_price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee_token: fee_token.into(),
    }.build())
}

#[frb(sync)]
pub fn sign_auto_deleveraging(signer: Signer, tx: AutoDeleveraging) -> Result<String> {
    let sig = signer.sign_auto_deleveraging(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn funding(
    account_id: u32,
    sub_account_id: u8,
    sub_account_nonce: u32,
    funding_account_ids: Vec<u32>,
    fee: String,
    fee_token: u16,
) -> Result<Funding> {
    let funding_account_ids = funding_account_ids
        .iter()
        .map(|id| (*id).into())
        .collect::<Vec<_>>();
    Ok(FundingBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        sub_account_nonce: sub_account_nonce.into(),
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee_token: fee_token.into(),
        funding_account_ids,
    }.build())
}

#[frb(sync)]
pub fn sign_funding(signer: Signer, tx: Funding) -> Result<String> {
    let sig = signer.sign_funding(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn liquidation(
    account_id: u32,
    sub_account_id: u8,
    sub_account_nonce: u32,
    contract_prices: Vec<ContractPrice>,
    margin_prices: Vec<SpotPriceInfo>,
    liquidation_account_id: u32,
    fee: String,
    fee_token: u16,
) -> Result<Liquidation> {
    Ok(LiquidationBuilder {
        account_id: account_id.into(),
        sub_account_id: sub_account_id.into(),
        sub_account_nonce: sub_account_nonce.into(),
        contract_prices,
        margin_prices,
        liquidation_account_id: liquidation_account_id.into(),
        fee: BigUint::from_str(&fee).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        fee_token: fee_token.into(),
    }.build())
}

#[frb(sync)]
pub fn sign_liquidation(signer: Signer, tx: Liquidation) -> Result<String> {
    let sig = signer.sign_liquidation(tx)?;
    Ok(to_string(&sig.tx)?)
}

#[frb(sync)]
pub fn parameter_fee_account(account_id: u32) -> Result<Parameter> {
    Ok(Parameter::FeeAccount { account_id: account_id.into() })
}

#[frb(sync)]
pub fn parameter_insurance_fund_account(account_id: u32) -> Result<Parameter> {
    Ok(Parameter::InsuranceFundAccount { account_id: account_id.into() })
}

#[frb(sync)]
pub fn parameter_margin_info(margin_id: u8, symbol: String, token_id: u32, ratio: u8) -> Result<Parameter> {
    Ok(Parameter::MarginInfo {
        margin_id: margin_id.into(),
        symbol,
        token_id: token_id.into(),
        ratio,
    })
}

#[frb(sync)]
pub fn parameter_funding_info(pair_id: u16, price: String, funding_rate: i16) -> Result<FundingInfo> {
    Ok(FundingInfo {
        pair_id: pair_id.into(),
        price: BigUint::from_str(&price).map_err(|e| InvalidBigIntStr(e.to_string()))?,
        funding_rate,
    })
}

#[frb(sync)]
pub fn parameter_funding_infos(infos: Vec<FundingInfo>) -> Result<Parameter> {
    Ok(Parameter::FundingInfos {infos})
}

#[frb(sync)]
pub fn parameter_contract_info(
    pair_id: u16,
    symbol: String,
    initial_margin_rate: u16,
    maintenance_margin_rate: u16,
) -> Result<Parameter> {
    Ok(Parameter::ContractInfo {
        pair_id: pair_id.into(),
        symbol,
        initial_margin_rate,
        maintenance_margin_rate,
    })
}

#[frb(sync)]
pub fn update_global_var(
    from_chain_id: u8,
    sub_account_id: u8,
    parameter: Parameter,
    serial_id: f64,
) -> Result<String> {
    let tx = UpdateGlobalVarBuilder {
        from_chain_id: from_chain_id.into(),
        sub_account_id: sub_account_id.into(),
        parameter: parameter.into(),
        serial_id: serial_id as u64,
    }.build();
    Ok(to_string(&tx)?)
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
