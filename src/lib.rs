pub use handler::*;
#[cfg(target_family = "wasm")]
pub use ic::*;
pub use interface::*;
pub use mock::*;

mod handler;
#[cfg(target_family = "wasm")]
mod ic;
mod inject;
mod interface;
mod mock;

/// async_std::test to be used for async tests when not targeting WASM.
#[cfg(not(target_family = "wasm"))]
pub use async_std::test as async_test;
pub use ic_cdk::api::call::{CallResult, RejectionCode};
pub use ic_cdk::export::candid;
pub use ic_cdk::export::Principal;
pub use ic_cdk_macros as macros;

/// A set of mock principal IDs useful for testing.
#[cfg(not(target_family = "wasm"))]
pub mod mock_principals {
    use crate::Principal;

    #[inline]
    pub fn alice() -> Principal {
        Principal::from_text("sgymv-uiaaa-aaaaa-aaaia-cai").unwrap()
    }

    #[inline]
    pub fn bob() -> Principal {
        Principal::from_text("ai7t5-aibaq-aaaaa-aaaaa-c").unwrap()
    }

    #[inline]
    pub fn john() -> Principal {
        Principal::from_text("hozae-racaq-aaaaa-aaaaa-c").unwrap()
    }

    #[inline]
    pub fn xtc() -> Principal {
        Principal::from_text("aanaa-xaaaa-aaaah-aaeiq-cai").unwrap()
    }
}

/// The type definition of common canisters on the Internet Computer.
pub mod interfaces;

/// Return the IC context depending on the build target.
#[inline(always)]
pub fn get_context() -> &'static mut impl Context {
    #[cfg(not(target_family = "wasm"))]
    return inject::get_context();
    #[cfg(target_family = "wasm")]
    return IcContext::context();
}
