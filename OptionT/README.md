## Option<T>


## Syntax

### Default Option<T>

```rust
pub enum Option<T> {
    Some(T),
    None
};
```
### Match

```rust
match an_option_value {
    Some(T) => { ... },
    None    => { ... }
}
```
## Option<T>

- Option<T> sử dụng để bắt lỗi chương trình
- Cấu trúc của một **Option<T>** và nếu thiếu 1 trong hai quá trình biên dịch sẽ lỗi
    - Some(T)
    - None
- Do đó khi đã cài đặt xong **Option<T>** thì đảm bảo chương trình không có lỗi

