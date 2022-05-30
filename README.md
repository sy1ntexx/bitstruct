# Bitstruct

## Example
```rs
bitstruct::bitstruct! {
    #[derive(Default)]
    struct Va : u64 {
        page: 0..=11,
        pt: 12..=20,
        pd: 21..=29,
        pdpt: 30..=38,
        pml4: 39..=47
    }
}
```

## Installation
```toml
[dependencies.bitstruct]
git = "https://github.com/sy1ntexx/bistruct"
```