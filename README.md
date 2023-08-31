# cardbot

## build
```
cargo build
```

## moxfield

if you have a (public) moxfield deck id, you can use it to query moxfield.
```
target/debug/cardctl -m <deck_id>
```
protip: try piping the json output to jq for prettier json.
