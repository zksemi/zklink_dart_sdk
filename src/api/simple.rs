use std::str::FromStr;
use std::time::Instant;
use anyhow::Result;
use serde_json::to_string;
use flutter_rust_bridge::frb;
use zklink_sdk_signers::eth_signer::H256;
use zklink_sdk_signers::starknet_signer::StarkEcdsaSignature;
use zklink_sdk_signers::zklink_signer::ZkLinkSigner;
use zklink_sdk_signers::{eth_signer::PackedEthSignature, zklink_signer::PubKeyHash};
use zklink_sdk_types::basic_types::ZkLinkAddress;
use zklink_sdk_types::tx_builder::*;
use zklink_sdk_types::tx_type::change_pubkey::{ChangePubKeyAuthData, Create2Data};
use zklink_sdk_types::tx_type::contract::*;
use zklink_sdk_types::tx_type::forced_exit::ForcedExit;
use zklink_sdk_types::tx_type::order_matching::{Order, OrderMatching};
use zklink_sdk_types::tx_type::transfer::Transfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw;
use zklink_sdk_types::{basic_types::BigUint, tx_type::change_pubkey::ChangePubKey};
use zklink_sdk_types::prelude::ChangePubKeyBuilder;
use zklink_sdk_interface::signer::{Signer, L1SignerType};
use zklink_sdk_wallet::eth::EthTxOption;
use zklink_sdk_wallet::wallet::Wallet;

#[frb(opaque)]
pub struct FFIZkLinkSigner {
    pub inner: ZkLinkSigner
}

impl FFIZkLinkSigner {
    #[frb(sync)]
    pub fn eth_sig(sig: String) -> Result<Self> {
        let signature = PackedEthSignature::from_hex(&sig)?;
        let seed = signature.serialize_packed();
        Ok(Self {
            inner: ZkLinkSigner::new_from_seed(&seed)?
        })
    }

    #[frb(sync)]
    pub fn starknet_sig(sig: String) -> Result<Self> {
        let signature = StarkEcdsaSignature::from_hex(&sig)?;
        let seed = signature.to_bytes_be();
        Ok(Self {
            inner: ZkLinkSigner::new_from_seed(&seed)?
        })
    }

    #[frb(sync)]
    pub fn get_pubkey(&self) -> String {
        self.inner.public_key().as_hex()
    }

    #[frb(sync)]
    pub fn get_pubkey_hash(&self) -> String {
        self.inner.public_key().public_key_hash().as_hex()
    }
}

#[frb(opaque)]
pub struct FFISigner {
    pub inner: Signer
}

impl FFISigner {
    #[frb(sync)]
    pub fn eth_signer(eth_private_key: String) -> Result<Self> {
        Ok(Self {
            inner: Signer::new(&eth_private_key, L1SignerType::Eth)?
        })
    }
    
    #[frb(sync)]
    pub fn starknet_signer(
        eth_private_key: String,
        starknet_chain_id: String,
        starknet_addr: String
    ) -> Result<Self> {
        let signer_type = L1SignerType::Starknet {
            chain_id: starknet_chain_id,
            address: starknet_addr,
        };
        Ok(Self {
            inner: Signer::new(&eth_private_key, signer_type)?
        })
    }

    #[frb(sync)]
    pub fn sign_change_pubkey_with_onchain(
        &self,
        tx: FFIChangePubKey,
    ) -> Result<String> {
        let sig = self.inner.sign_change_pubkey_with_onchain_auth_data(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }
    
    #[frb(sync)]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(
        &self,
        tx: FFIChangePubKey,
    ) -> Result<String> {
        let sig = self.inner.sign_change_pubkey_with_eth_ecdsa_auth(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }
    
    #[frb(sync)]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: FFIChangePubKey,
        creator_address: String,
        salt_arg: String,
        code_hash: String,
    ) -> Result<String> {
        let create2_data = Create2Data {
            creator_address: ZkLinkAddress::from_hex(&creator_address)?,
            code_hash: H256::from_str(&code_hash)?,
            salt_arg: H256::from_str(&salt_arg)?,
        };
        let sig = self.inner.sign_change_pubkey_with_create2data_auth(tx.inner, create2_data)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_transfer(
        &self,
        tx: FFITransfer,
        token_symbol: String,
        chain_id: Option<String>,
        addr: Option<String>,
    ) -> Result<String> {
        let sig = self.inner.sign_transfer(tx.inner, &token_symbol, chain_id, addr)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_withdraw(
        &self,
        tx: FFIWithdraw,
        token_symbol: String,
        chain_id: Option<String>,
        addr: Option<String>,
    ) -> Result<String> {
        let sig = self.inner.sign_withdraw(tx.inner, &token_symbol, chain_id, addr)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_forced_exit(&self, tx: FFIForcedExit) -> Result<String> {
        let sig = self.inner.sign_forced_exit(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn create_signed_order(&self, order: Order) -> Result<Order> {
        Ok(self.inner.create_signed_order(&order)?)
    }

    #[frb(sync)]
    pub fn sign_order_matching(&self, tx: FFIOrderMatching) -> Result<String> {
        let sig = self.inner.sign_order_matching(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn create_signed_contract(&self, contract: Contract) -> Result<Contract> {
        Ok(self.inner.create_signed_contract(&contract)?)
    }
    
    #[frb(sync)]
    pub fn sign_contract_matching(&self, tx: FFIContractMatching) -> Result<String> {
        let sig = self.inner.sign_contract_matching(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_auto_deleveraging(&self, tx: FFIAutoDeleveraging) -> Result<String> {
        let sig = self.inner.sign_auto_deleveraging(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_funding(&self, tx: FFIFunding) -> Result<String> {
        let sig = self.inner.sign_funding(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_liquidation(&self, tx: FFILiquidation) -> Result<String> {
        let sig = self.inner.sign_liquidation(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }
}

#[frb(opaque)]
pub struct FFIChangePubKey {
    pub inner: ChangePubKey
}

impl FFIChangePubKey {
    #[frb(sync)]
    pub fn new(
        chain_id: u8,
        account_id: u32,
        sub_account_id: u8,
        new_pubkey_hash: String,
        fee_token: u32,
        fee: String,
        nonce: u32,
        eth_signature: Option<String>,
        ts: Option<u32>,
    ) -> Result<Self> {
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
        Ok(Self { 
            inner: ChangePubKeyBuilder {
                chain_id: chain_id.into(),
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                new_pubkey_hash: PubKeyHash::from_hex(&new_pubkey_hash)?,
                fee_token: fee_token.into(),
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                eth_signature,
                timestamp: ts.into(),
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_eip712_request_payload(
        &self,
        chain_id: u32,
        address: String,
    ) -> Result<String> {
        let eth_data = self.inner.to_eip712_request_payload(
            chain_id,
            &ZkLinkAddress::from_hex(&address)?,
        )?;
        Ok(to_string(&eth_data)?)
    }

    #[frb(sync)]
    pub fn get_eth_sign_msg(
        &self,
        nonce: u32,
        account_id: u32,
    ) -> String {
        format!(
            "ChangePubKey\nPubKeyHash: {}\nNonce: {}\nAccountId: {}",
            self.inner.new_pk_hash.as_hex(),
            nonce,
            account_id
        )
    }

    #[frb(sync)]
    pub fn set_eth_authdata(&mut self, sig: String) -> Result<()> {
        let eth_signature = PackedEthSignature::from_hex(&sig)?;
        let eth_authdata = ChangePubKeyAuthData::EthECDSA { eth_signature };
        self.inner.eth_auth_data = eth_authdata;
        Ok(())
    }

    #[frb(sync)]
    pub fn sign(&mut self, zklink_signer: FFIZkLinkSigner) -> Result<()> {
        Ok(self.inner.sign(&zklink_signer.inner)?)
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFITransfer {
    pub inner: Transfer
}

impl FFITransfer {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        to_address: String,
        from_sub_account_id: u8,
        to_sub_account_id: u8,
        token: u32,
        fee: String,
        amount: String,
        nonce: u32,
        ts: Option<u32>,
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            Instant::now().elapsed().as_secs() as u32
        };
        Ok(Self { 
            inner: TransferBuilder {
                account_id: account_id.into(),
                to_address: ZkLinkAddress::from_hex(&to_address)?,
                from_sub_account_id: from_sub_account_id.into(),
                to_sub_account_id: to_sub_account_id.into(),
                token: token.into(),
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                timestamp: ts.into(),
                amount: BigUint::from_str(&amount)?,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFIWithdraw {
    pub inner: Withdraw
}

impl FFIWithdraw {
    #[frb(sync)]
    pub fn new(
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
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            Instant::now().elapsed().as_secs() as u32
        };
        let data_hash = if let Some(data_hash) = data_hash {
            Some(H256::from_str(&data_hash)?)
        } else {
            None
        };
        Ok(Self { 
            inner: WithdrawBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                to_chain_id: to_chain_id.into(),
                to_address: ZkLinkAddress::from_hex(&to_address)?,
                l2_source_token: l2_source_token.into(),
                l1_target_token: l1_target_token.into(),
                amount: BigUint::from_str(&amount)?,
                data_hash,
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                withdraw_to_l1,
                withdraw_fee_ratio,
                timestamp: ts.into(),
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFIForcedExit {
    pub inner: ForcedExit
}

impl FFIForcedExit {
    #[frb(sync)]
    pub fn new(
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
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            Instant::now().elapsed().as_secs() as u32
        };
        Ok(Self { 
            inner: ForcedExitBuilder {
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
                exit_amount: BigUint::from_str(&exit_amount)?,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(sync)]
pub fn contract_price(pair_id: u16, market_price: String) -> Result<ContractPrice> {
    Ok(ContractPrice {
        pair_id: pair_id.into(),
        market_price: BigUint::from_str(&market_price)?,
    })
}

#[frb(sync)]
pub fn spot_price(token_id: u32, price: String) -> Result<SpotPriceInfo> {
    Ok(SpotPriceInfo {
        token_id: token_id.into(),
        price: BigUint::from_str(&price)?,
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
        amount: BigUint::from_str(&amount)?,
        price: BigUint::from_str(&price)?,
        is_sell: is_sell as u8,
        fee_rates: [maker_fee_rate, taker_fee_rate],
        has_subsidy: has_subsidy as u8,
        signature: Default::default(),
    })
}

#[frb(opaque)]
pub struct FFIOrderMatching {
    pub inner: OrderMatching
}

impl FFIOrderMatching {
    #[frb(sync)]
    pub fn new(
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
    ) -> Result<Self> {
        Ok(Self { 
            inner: OrderMatchingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                taker,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                expect_base_amount: BigUint::from_str(&expect_base_amount)?,
                maker,
                expect_quote_amount: BigUint::from_str(&expect_quote_amount)?,
                contract_prices,
                margin_prices,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
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
        size: BigUint::from_str(&size)?,
        price: BigUint::from_str(&price)?,
        direction,
        maker_fee_rate,
        taker_fee_rate,
        has_subsidy,
    }.build())
}

#[frb(opaque)]
pub struct FFIContractMatching {
    pub inner: ContractMatching
}

impl FFIContractMatching {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        taker: Contract,
        maker: Vec<Contract>,
        fee: String,
        fee_token: u16,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
    ) -> Result<Self> {
        Ok(Self { 
            inner: ContractMatchingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                taker,
                maker,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                contract_prices,
                margin_prices,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFIAutoDeleveraging {
    pub inner: AutoDeleveraging
}

impl FFIAutoDeleveraging {
    #[frb(sync)]
    pub fn new(
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
    ) -> Result<Self> {
        Ok(Self { 
            inner: AutoDeleveragingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                contract_prices,
                margin_prices,
                adl_account_id: adl_account_id.into(),
                pair_id: pair_id.into(),
                adl_size: BigUint::from_str(&adl_size)?,
                adl_price: BigUint::from_str(&adl_price)?,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFIFunding {
    pub inner: Funding
}

impl FFIFunding {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        funding_account_ids: Vec<u32>,
        fee: String,
        fee_token: u16,
    ) -> Result<Self> {
        let funding_account_ids = funding_account_ids
            .iter()
            .map(|id| (*id).into())
            .collect::<Vec<_>>();
        Ok(Self { 
            inner: FundingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                funding_account_ids,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFILiquidation {
    pub inner: Liquidation
}

impl FFILiquidation {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
        liquidation_account_id: u32,
        fee: String,
        fee_token: u16,
    ) -> Result<Self> {
        Ok(Self { 
            inner: LiquidationBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                contract_prices,
                margin_prices,
                liquidation_account_id: liquidation_account_id.into(),
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
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
        price: BigUint::from_str(&price)?,
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

#[frb(opaque)]
pub struct FFIUpdateGlobalVar {
    pub inner: UpdateGlobalVar
}

impl FFIUpdateGlobalVar {
    #[frb(sync)]
    pub fn new(
        from_chain_id: u8,
        sub_account_id: u8,
        parameter: Parameter,
        serial_id: f64,
    ) -> Result<Self> {
        Ok(Self { 
            inner: UpdateGlobalVarBuilder {
                from_chain_id: from_chain_id.into(),
                sub_account_id: sub_account_id.into(),
                parameter: parameter.into(),
                serial_id: serial_id as u64,
            }.build()
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct FFIEthTxOption {
    pub inner: EthTxOption
}

impl FFIEthTxOption {
    #[frb(sync)]
    pub fn new(
        is_support_eip1559: bool,
        to: String,
        nonce: Option<f64>,
        value: Option<String>,
        gas: Option<f64>,
        gas_price: Option<String>,
    ) -> Result<Self> {
        let value = if let Some(v) = value {
            Some(BigUint::from_str(&v)?)
        } else {
            None
        };
        let gas_price = if let Some(g) = gas_price {
            Some(BigUint::from_str(&g)?)
        } else {
            None
        };
        Ok(Self { 
            inner: EthTxOption {
                is_support_eip1559,
                to: ZkLinkAddress::from_hex(&to)?,
                nonce: nonce.map(|n| n as u64),
                value,
                gas: gas.map(|g| g as u64),
                gas_price,
            }
        })
    }
}

#[frb(opaque)]
pub struct FFIWallet {
    pub inner: Wallet
}

impl FFIWallet {
    #[frb(sync)]
    pub fn new(url: String, private_key: String) -> Result<Self> {
        Ok(Self {
            inner: Wallet::new(&url, &private_key)
        })
    }

    pub async fn get_balance(&self) -> Result<String> {
        let balance = self.inner.get_balance().await?;
        Ok(balance.to_string())
    }

    pub async fn get_nonce(&self, block_number: String) -> Result<f64> {
        let nonce = self.inner.get_nonce(block_number).await?;
        Ok(nonce.as_u64() as f64)
    }

    pub async fn get_deposit_fee(&self, eth_params: FFIEthTxOption) -> Result<String> {
        let fee = self.inner.get_fee(eth_params.inner).await?;
        Ok(fee.to_string())
    }

    pub async fn wait_for_transaction(
        &self,
        tx_hash: String,
        timeout: Option<u32>,
    ) -> Result<u8> {
        let tx_hash = H256::from_str(&tx_hash)?;
        let status = self.inner.wait_for_transaction(tx_hash, timeout).await?;
        Ok(status as u8)
    }

    pub async fn approve_erc20(
        &self,
        contract: String,
        amount: String,
        eth_params: FFIEthTxOption,
    ) -> Result<String> {
        let contract = ZkLinkAddress::from_hex(&contract)?;
        let amount = BigUint::from_str(&amount)?;
        let tx_hash = self.inner.approve_erc20(contract, amount, eth_params.inner).await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn deposit_erc20(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        token_addr: String,
        amount: String,
        mapping: bool,
        eth_params: FFIEthTxOption,
        is_gateway: bool,
    ) -> Result<String> {
        let deposit_to = ZkLinkAddress::from_hex(&deposit_to)?;
        let token_addr = ZkLinkAddress::from_hex(&token_addr)?;
        let amount = BigUint::from_str(&amount)?;
        let tx_hash = if !is_gateway {
            self.inner.deposit_erc20_to_layer1(
                sub_account_id,
                deposit_to,
                token_addr,
                amount,
                mapping,
                eth_params.inner,
            )
            .await?
        } else {
            self.inner.deposit_erc20_to_gateway(
                sub_account_id,
                deposit_to,
                token_addr,
                amount,
                mapping,
                eth_params.inner,
            )
            .await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn deposit_eth(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        eth_params: FFIEthTxOption,
        is_gateway: bool,
    ) -> Result<String> {
        let deposit_to = ZkLinkAddress::from_hex(&deposit_to)?;
        let tx_hash = if !is_gateway {
            self.inner.deposit_eth_to_layer1(sub_account_id, deposit_to, eth_params.inner).await?
        } else {
            self.inner.deposit_eth_to_gateway(sub_account_id, deposit_to, eth_params.inner).await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn set_auth_pubkey_hash(
        &self,
        nonce: f64,
        new_pubkey_hash: String,
        eth_params: FFIEthTxOption,
    ) -> Result<String> {
        let new_pubkey_hash = PubKeyHash::from_hex(&new_pubkey_hash)?;
        let tx_hash = self.inner.set_auth_pubkey_hash(nonce as u64, new_pubkey_hash, eth_params.inner).await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn full_exit(
        &self,
        account_id: u32,
        sub_account_id: u8,
        token_id: u16,
        mapping: bool,
        eth_params: FFIEthTxOption,
    ) -> Result<String> {
        let tx_hash = self.inner.full_exit(account_id, sub_account_id, token_id, mapping, eth_params.inner).await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
