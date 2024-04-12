// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.30.

// Section: imports

use super::*;
use crate::api::simple::*;
use flutter_rust_bridge::for_generated::byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use flutter_rust_bridge::for_generated::transform_result_dco;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: boilerplate

flutter_rust_bridge::frb_generated_boilerplate_io!();

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockAutoDeleveraging(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<AutoDeleveraging>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockAutoDeleveraging(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<AutoDeleveraging>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockChangePubKey(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ChangePubKey>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockChangePubKey(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ChangePubKey>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContract(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Contract>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContract(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Contract>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContractMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ContractMatching>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContractMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ContractMatching>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContractPrice(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ContractPrice>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockContractPrice(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ContractPrice>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockForcedExit(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ForcedExit>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockForcedExit(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<ForcedExit>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFunding(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Funding>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFunding(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Funding>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFundingInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FundingInfo>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockFundingInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<FundingInfo>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLiquidation(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Liquidation>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockLiquidation(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Liquidation>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOrder(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Order>>::increment_strong_count(
        ptr as _,
    );
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOrder(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Order>>::decrement_strong_count(
        ptr as _,
    );
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOrderMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OrderMatching>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockOrderMatching(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<OrderMatching>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockParameter(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Parameter>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockParameter(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Parameter>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Signer>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSigner(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Signer>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSpotPriceInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<SpotPriceInfo>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockSpotPriceInfo(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<SpotPriceInfo>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockTransfer(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Transfer>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockTransfer(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Transfer>>::decrement_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockWithdraw(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Withdraw>>::increment_strong_count(ptr as _);
}

#[no_mangle]
pub extern "C" fn frbgen_my_app_rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedrust_asyncRwLockWithdraw(
    ptr: *const std::ffi::c_void,
) {
    MoiArc::<flutter_rust_bridge::for_generated::rust_async::RwLock<Withdraw>>::decrement_strong_count(ptr as _);
}