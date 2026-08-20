```bash
sudo pacman -S git git-lfs rust git lfs install
```
```bash
git clone https://github.com/supertone-inc/supertonic.git
cd supertonic
```
```bash
cargo build --release
```
```bash
git clone https://huggingface.co/Supertone/supertonic-3 assets
```
```bash
mkdir -p ~/.local/share/supertonic
cp assets ~/.local/share/supertonic/
```
```bash
cp target/release/supertonic ~/.local/bin/
```