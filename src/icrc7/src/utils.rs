use candid::Principal;
use icrc_ledger_types::icrc::generic_value::Value;
use icrc_ledger_types::icrc1::account::{Account, Subaccount, DEFAULT_SUBACCOUNT};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn account_transformer(account: Account) -> Account {
    if let Some(_) = account.subaccount {
        account
    } else {
        Account {
            owner: account.owner,
            subaccount: Some(DEFAULT_SUBACCOUNT.clone()),
        }
    }
}

pub fn default_account(owner: &Principal) -> Account {
    Account {
        owner: owner.clone(),
        subaccount: Some(DEFAULT_SUBACCOUNT.clone()),
    }
}

pub fn burn_subaccount() -> Subaccount {
    let mut bytes = [0; 32];
    let slice = b"BURN SUBACCOUNT";
    bytes[0..15].copy_from_slice(slice);
    bytes
}

pub fn burn_account() -> Account {
    Account {
        owner: ic_cdk::api::id(),
        subaccount: Some(burn_subaccount()),
    }
}

pub fn hash_icrc_value(value: &Value) -> Vec<u8> {
    let mut hasher = DefaultHasher::new();
    encode_value(value, &mut hasher);
    let hash = hasher.finish();

    // Convert hash u64 to little-endian byte array
    let mut result = Vec::new();
    result.extend_from_slice(&hash.to_le_bytes());
    result
}

fn encode_value(value: &Value, hasher: &mut DefaultHasher) {
    match value {
        Value::Blob(b) => {
            hasher.write(b);
        }
        Value::Text(t) => {
            t.hash(hasher);
        }
        Value::Nat(n) => {
            n.hash(hasher);
        }
        Value::Nat64(n) => {
            n.hash(hasher);
        }
        Value::Int(i) => {
            i.hash(hasher);
        }
        Value::Array(a) => {
            for v in a {
                encode_value(v, hasher);
            }
        }
        Value::Map(m) => {
            let mut entries: Vec<(String, Value)> =
                m.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            for (k, v) in entries {
                k.hash(hasher);
                encode_value(&v, hasher);
            }
        }
    }
}
