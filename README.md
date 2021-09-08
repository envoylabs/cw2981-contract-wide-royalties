![Build and Test](https://github.com/envoylabs/cw2981-contract-wide-royalties/actions/workflows/build_and_test.yml/badge.svg)

# CW2981 Contract-wide Royalties

An example of porting EIP2981 to implement royalties at a per-contract level.

Exposes a new query message type:

```rust
// Should be called on sale to see if royalties are owed
// by the marketplace selling the NFT.
// See https://eips.ethereum.org/EIPS/eip-2981
RoyaltyInfo {
    token_id: String,
    // the denom of this sale must also be the denom returned by RoyaltiesInfoResponse
    sale_price: Coin,
},
```

Note that as this is contract-wide, `token_id` is required but ignored.
