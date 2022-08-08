# puball

A simple API that help you generate struct with all fields public.

## Motivation

A friend of mine wrote a huge struct with 71 `pub` keywords.
It's too hard to write so much `pub` keywords.
Especially when you realize that your forgot to add those visbilty
after you finish the sturct design.

## Usage

```toml
# Cargo.toml

[dependencies]
puball = "0.1"
```

```rust
mod child {
    use puball::pub_all;

    pub_all!{
        pub struct NoPrivacy {
            a: i32,
            b: String,
            c: bool,
        }
    }
}

fn main() {
    use child::NoPrivacy;

    let np = NoPrivacy {
        a: 1,
        b: String::new(),
        c: true,
    };

    assert_eq!(1, np.a);
    assert!(np.c);
}
```
