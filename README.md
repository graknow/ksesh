# ksesh
`ksesh` is a simple Rust script to assist with managing kitty session files.  It allows easy creation, loading, and deletion of session files.

## Installation

### Prerequisites
The rust toolkit should be installed. Details on that can be found on the [Rust Website](https://www.rust-lang.org/tools/install).

### Procedure
Currently, ksesh can be installed by doing the following:
```
git clone https://github.com/graknow/ksesh.git
cd ksesh
cargo build --release
```
After building, you can place the executable in any place where you typically put shell scripts.

## Usage

### Save
Kitty session files are automatically stored in ~/.config/kitty/sessions/

```
// Saves to ~/.config/kitty/sessions/test/session.kitty
ksesh -S test/session
```
### Load
Kitty sessions are loaded from ~/.config/kitty/sessions/

```
// Loads ~/.config/kitty/sessions/test/load/session.kitty
ksesh -L test/load/session
```

### Delete
Deletes session from ~/.config/kitty/sessions/

```
// Deletes ~/.config/kitty/sessions/delete.kitty
ksesh -D delete
```