## 1. Github code
https://github.com/duguorong009/substrate-simple-custom-pallet/tree/main/pallets/custom

## 2. Benchmark Analysis
https://github.com/duguorong009/substrate-simple-custom-pallet/blob/main/pallets/all-weight.rs

## 3. Test Cases
https://github.com/duguorong009/substrate-simple-custom-pallet/blob/main/pallets/custom/src/tests.rs

There are 5 test cases: 
  - Work as expected when `add/remove_member` from `club(s)`
  - Invoke `error` when `non-Root` account calls
  - Invoke `error` when `no club exists` for `remove_member`
  - Invoke `error` when `no member exists` for `remove_member`
  - Invoke `error` when `already member exists` for `add_member`