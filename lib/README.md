# kos
kos (Key, Offset, and Salt) is a quick encryption protocal that uses math to encrypt and decrypt.
# whats new in lib?
(3.1.3) - minor bug fix
# features
kos 3.0.0+ allows for the user to implement this protocal in their program.
using lib is the vanilla experience.
using kos includes a basic ui.
# demo
Web demo [here](https://replit.com/@EliThrash/kos)
# example
```rust
use kosalt;
fn main() {
    kosalt::gen();
    kosalt::encrypt("This is super cool!");
    let message = kosalt::decrypt();
    println!("{message}");
}
```
