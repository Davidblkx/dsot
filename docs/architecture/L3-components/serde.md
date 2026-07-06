# Binary Serialization Component (`dsot_serde`)

The `dsot_serde` crate is a lightweight utility container providing standardized, high-performance binary serialization and deserialization across the DSOT workspace.

---

## Responsibility

- **Format Standardization:** Establishes MessagePack (`rmp-serde`) as the canonical binary serialization format across all DSOT crates.
- **Boilerplate Reduction:** Provides helper structs (`BinarySerde`) and declarative macros (`serde_binary!`) to equip domain models and network messages with clean binary conversion methods.

---

## Core Interfaces

### 1. `BinarySerde`

A stateless helper wrapping `rmp_serde` encoding and decoding routines with workspace-standard error handling:

```rust
pub struct BinarySerde;

impl BinarySerde {
    pub fn serialize<T: serde::Serialize>(value: &T) -> Result<Vec<u8>>;
    pub fn deserialize<T: serde::de::DeserializeOwned>(data: &[u8]) -> Result<T>;
}
```

### 2. `serde_binary!` Macro

A declarative macro that generates standard `.to_binary()` and `.from_binary()` methods on any struct or enum implementing Serde's `Serialize` and `Deserialize`:

```rust
#[macro_export]
macro_rules! serde_binary {
    ($id:ident) => {
        impl $id {
            pub fn to_binary(&self) -> Result<Vec<u8>> {
                let v = $crate::BinarySerde::serialize(self)?;
                Ok(v)
            }

            pub fn from_binary(data: &[u8]) -> Result<Self> {
                let v = $crate::BinarySerde::deserialize(data)?;
                Ok(v)
            }
        }
    };
}
```

---

## Usage Across Workspace

- **`dsot_db_sync`**: Uses `BinarySerde` to serialize and deserialize entity rows (`SyncOperation::Create`, journal entries, and diff payloads) into compact byte vectors stored in SQLite and redb.
- **`dsot_network`**: Uses `BinarySerde` within `NetworkReader` and `NetworkWriter` to encode and decode framed QUIC network stream messages (`DBSyncMessage`, machine info payloads).
