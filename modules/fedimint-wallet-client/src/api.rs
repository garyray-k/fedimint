use bitcoin::Address;
use fedimint_core::api::{FederationApiExt, FederationResult, IFederationApi};
use fedimint_core::core::LEGACY_HARDCODED_INSTANCE_ID_WALLET;
use fedimint_core::module::ApiRequestErased;
use fedimint_core::query::EventuallyConsistent;
use fedimint_core::task::{MaybeSend, MaybeSync};
use fedimint_core::{apply, async_trait_maybe_send, NumPeers};
use fedimint_wallet_common::PegOutFees;

#[apply(async_trait_maybe_send!)]
pub trait WalletFederationApi {
    async fn fetch_consensus_block_height(&self) -> FederationResult<u64>;
    async fn fetch_peg_out_fees(
        &self,
        address: &Address,
        amount: bitcoin::Amount,
    ) -> FederationResult<Option<PegOutFees>>;
}

#[apply(async_trait_maybe_send!)]
impl<T: ?Sized> WalletFederationApi for T
where
    T: IFederationApi + MaybeSend + MaybeSync + 'static,
{
    async fn fetch_consensus_block_height(&self) -> FederationResult<u64> {
        self.request_with_strategy(
            EventuallyConsistent::new(self.all_members().one_honest()),
            format!("module_{LEGACY_HARDCODED_INSTANCE_ID_WALLET}_block_height"),
            ApiRequestErased::default(),
        )
        .await
    }

    async fn fetch_peg_out_fees(
        &self,
        address: &Address,
        amount: bitcoin::Amount,
    ) -> FederationResult<Option<PegOutFees>> {
        self.request_eventually_consistent(
            format!("module_{LEGACY_HARDCODED_INSTANCE_ID_WALLET}_peg_out_fees"),
            ApiRequestErased::new((address, amount.to_sat())),
        )
        .await
    }
}
