# <p align="center"> ![Image](imgs/FullLogo.png)</p>

## TUI-based space strategy game

![Image](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
<a href="https://discord.com/invite/wP3mtUtKFz">![Image](https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white)</a>

## 🧐 Features
- Unique UI design
- A research tree that unlocks new ship modules over time
- Colony management with resource ticks and building construction
- Design and build a fleet of armed starships from unlocked engine and weapon modules
- Fleet movement between the star and planets in your system
- Tick-based combat against a scripted enemy guarding the outer system — rarely (25%) accompanied by a much tougher escort
- The enemy advances on your colony over time if left unopposed — losing it ends the game
- Real orbital mechanics for celestial bodies (orbit radius/period derived from orbital velocity)

## 🎮 How to play
1. Open the **Research** tab and research an engine (e.g. Ion Drive) and a weapon (e.g. Ion Cannon)
2. Open the **Colonies** tab to grow your capital colony and queue up mines and factories — a ship needs both Engine Nozzles (via a Heat Resistant Alloy factory chain) and Microprocessors (via a Superconductors + Electronics chain) to build
3. Once research unlocks a module, open the **Ship modules** tab, select its type, then the module, and press Enter to finalize a design — you need both an engine and a weapon design
4. Press `<Alt-r>` in the Ship modules tab to build a ship from your current designs — press it again any time to build another and grow your fleet, even mid-fight
5. Your fleet starts at your home planet, while the enemy guards the outermost planet in the system — open the **System View** tab, press `<Alt-s>` and the arrow keys to highlight a body, then `<Alt-r>` to send your fleet there. Travel takes time
6. Once your fleet reaches the enemy's location, combat resolves automatically each tick — every surviving ship fires at once, stacking damage, while the enemy's counter-fire always hits whichever ship is at the front of the fleet until it's destroyed. Sometimes a second, much tougher escort is guarding alongside the standard enemy — check the Shipyard panel's "Enemy fleet" line before you commit, since it keeps firing even while you're still working through the weaker one. Wipe out every enemy ship to win; if your whole fleet is destroyed first, build more ships and send them back to finish the job
7. Don't dawdle: if you never engage it, the enemy periodically advances toward your home planet (watch the **Fleet** panel in System View for a countdown) and, once it arrives, sieges your colony each tick until either you intercept it or the colony's HP hits 0 — losing the colony loses the game

## 🛠️ Installation

### From source
Requires [Rust](https://rustup.rs) (the pinned toolchain is `stable`, see `rust-toolchain.toml`):
```bash
git clone https://github.com/justwaters/astray.git
cd astray
cargo install --path .
```
This installs the `astray` binary to `~/.cargo/bin`, which is self-contained and can be freely moved or symlinked elsewhere. Make sure `~/.cargo/bin` is on your `PATH`, then run:
```bash
astray
```

### From a release
Once a version is tagged, the [CD workflow](.github/workflows/cd.yml) builds macOS, Linux, and Windows binaries and attaches them to the [GitHub Releases page](https://github.com/justwaters/astray/releases). Download the archive for your platform, unzip it, and run the extracted binary directly — no separate `assets/` folder needed.

## 🍰 Contributing
Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## ➤ License
Distributed under the MIT License. See [LICENSE](LICENSE) for more information.

## ❤️ Support
A simple star to this project repo is enough to keep me motivated on this project for days.
If you are feeling extra-generous today, you can donate some money to make 
me really happy - https://donate.stream/astray

## Screenshots
![Screenshot 1](imgs/screenshot_1.png)
