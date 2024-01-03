# Finschia/wasmd#114 mini repro

1. `beaker wasm build`
2. `beaker wasm store-code delegator --signer-account test1 -n localnet`
3. `beaker wasm instantiate delegator --signer-account test1 -n localnet --raw '{"validator":"linkvaloper1twsfmuj28ndph54k4nw8crwu8h9c8mh33lyrp8"}' --no-proposal-sync -f 10000cony`
4. `beaker wasm execute delegator --signer-account test1 -n localnet --raw '{"debug_set_withdraw_address":{}}'`
5. Check events

## expected events

```json
[
  {
    "events": [
      {
        "type": "execute",
        "attributes": [
          {
            "key": "_contract_address",
            "value": "link1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0skx2z7a"
          }
        ]
      },
      {
        "type": "message",
        "attributes": [
          {
            "key": "action",
            "value": "/cosmwasm.wasm.v1.MsgExecuteContract"
          },
          {
            "key": "sender",
            "value": "link1rvw82lxnwh6423clfxwcawhjp9slxxq8n23lee"
          },
          {
            "key": "module",
            "value": "wasm"
          }
        ]
      },
      {
        "type": "reply",
        "attributes": [
          {
            "key": "_contract_address",
            "value": "link1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0skx2z7a"
          }
        ]
      },
      {
        "type": "set_withdraw_address",
        "attributes": [
          {
            "key": "withdraw_address",
            "value": "link1cm0pgvsscsjveltaqucxh267vu2a3t60w7as0ydqgm"
          }
        ]
      },
      {
        "type": "wasm",
        "attributes": [
          {
            "key": "_contract_address",
            "value": "link1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0skx2z7a"
          },
          {
            "key": "method",
            "value": "execute"
          },
          {
            "key": "action",
            "value": "debug_set_withdraw_address"
          },
          {
            "key": "_contract_address",
            "value": "link1w27ekqvvtzfanfxnkw4jx2f8gdfeqwd3drkee3e64xat6phwjg0skx2z7a"
          },
          {
            "key": "action",
            "value": "debug_set_withdraw_address"
          },
          {
            "key": "reply_event_num",
            "value": "0"
          }
        ]
      }
    ]
  }
]
```
