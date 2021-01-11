# multikey-rs
Self Defining Public/Private/Secret Key format

## Implementing
multiformats/multikey
https://github.com/ipfs/specs/issues/58

## Format
`<key type><crypto type><size><key data>`
 - key type: 2 bit, first bit: asymmetric / symmetric, second bit: pub/priv (n/a for secret symmetric crypto)
 - crypto type: 6 bits, 64 possible algorithms for asymm/symm
 - size: [unsigned varint](https://github.com/multiformats/unsigned-varint)
 - key data: bytes of the key
