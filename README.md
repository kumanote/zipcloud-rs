# zipcloud-rs

> zipcloud-rs is a very simple library to fetch address information by zip code calling `zipcloud.ibsnet.co.jp` api.

## Installation

#### Dependencies

- [Rust with Cargo](http://rust-lang.org)

**rust-toolchain**

```text
1.59.0
```

#### Importing

**Cargo.toml**

```toml
[dependencies]
zipcloud = { version = "0.1.0", git = "https://github.com/kumanote/zipcloud-rs", branch = "main" }
```

**rust files**

```rust
use zipcloud;
```

## Usage

```rust
use zipcloud::Address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let zipcode = "100-0000";
    let address = zipcloud::fetch_address(zipcode).await.unwrap();
    assert!(address.is_some());
    assert_eq!(
        Address {
            address1: "東京都".to_owned(),
            address2: "千代田区".to_owned(),
            address3: "".to_owned(),
            kana1: "ﾄｳｷｮｳﾄ".to_owned(),
            kana2: "ﾁﾖﾀ\u{ff9e}ｸ".to_owned(),
            kana3: "".to_owned(),
            prefcode: "13".to_owned(),
            zipcode: "1000000".to_owned()
        },
        address.unwrap()
    );
    Ok(())
}
```
