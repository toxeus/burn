# burn

burn your bitcoins the proper way

## Usage

```sh
# find output you want to burn
bitcoin-cli listunspent
# replace vars with values from output of listunspent
tx=$(burn --txid $TXID --vout $VOUT --amount $AMOUNT --msg "some msg")
signed_tx=$(bitcoin-cli signrawtransactionwithwallet "$tx")
bitcoin-cli sendrawtransaction "$signed_tx"
```
