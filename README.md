# burn

burn your bitcoins the proper way

## Usage

```sh
# find output you want to burn
bitcoin-cli listunspent
# replace vars with values from output of listunspent
tx=$(burn --txid $TXID --vout $VOUT --amount $AMOUNT --msg "some msg")
signed_tx=$(bitcoin-cli signrawtransactionwithwallet "$tx" | jq -r .hex)
bitcoin-cli sendrawtransaction "$signed_tx"
```

The `jq` util used above can be found
[here](https://github.com/stedolan/jq.git).

You can easily inspect raw transactions.

```sh
certurium-cli decoderawtransaction "$tx"
certurium-cli decoderawtransaction "$signed_tx"
```
