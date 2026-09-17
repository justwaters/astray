# <p align="center"> ![Image](imgs/FullLogo.png)</p>

## TUI-based space strategy game

![Image](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
<a href="https://discord.com/invite/wP3mtUtKFz">![Image](https://img.shields.io/badge/Discord-5865F2?style=for-the-badge&logo=discord&logoColor=white)</a>

## 🧐 Features
- Unique UI design
- A research tree that unlocks new ship modules over time
- Colony management with resource ticks and building construction
- Design and build an armed starship from unlocked engine and weapon modules
- Fleet movement between the star and planets in your system
- Tick-based combat against a scripted enemy ship guarding the outer system
- The enemy advances on your colony over time if left unopposed — losing it ends the game
- Real orbital mechanics for celestial bodies (orbit radius/period derived from orbital velocity)

## 🎮 How to play
1. Open the **Research** tab and research an engine (e.g. Ion Drive) and a weapon (e.g. Ion Cannon)
2. Open the **Colonies** tab to grow your capital colony and queue up mines and factories — a ship needs both Engine Nozzles (via a Heat Resistant Alloy factory chain) and Microprocessors (via a Superconductors + Electronics chain) to build
3. Once research unlocks a module, open the **Ship modules** tab, select its type, then the module, and press Enter to finalize a design — you need both an engine and a weapon design
4. Press `<Alt-r>` in the Ship modules tab to build a ship from your current designs
5. Your fleet starts at your home planet, while the enemy guards the outermost planet in the system — open the **System View** tab, press `<Alt-s>` and the arrow keys to highlight a body, then `<Alt-r>` to send your fleet there. Travel takes time
6. Once your fleet reaches the enemy's location, combat resolves automatically each tick — destroy it to win. If your ship is destroyed first, build another and send it back to finish the job
7. Don't dawdle: if you never engage it, the enemy periodically advances toward your home planet (watch the **Fleet** panel in System View for a countdown) and, once it arrives, sieges your colony each tick until either you intercept it or the colony's HP hits 0 — losing the colony loses the game

## 🛠️ Installation
```
1. Download the latest release for your platform (macOS, Linux, or Windows) from GitHub
2. Unzip the archive
3. Run the astray executable — it's self-contained, so it can be moved or added to your PATH
```

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
