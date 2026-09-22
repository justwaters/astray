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
A how-to-play screen (goal, walkthrough, symbol/color legend, and keybindings) opens automatically the
first time you launch the game — press `<Enter>` or `<Esc>` to dismiss it, and `<Alt-h>` (or plain `h`)
to reopen it any time. Every Alt-shortcut below also has a plain-letter fallback (`<s>`, `<r>`, `<f>`,
`<h>`), since some terminals — notably macOS Terminal.app by default — don't send Option/Alt as a Meta
key, which would otherwise make the Alt-shortcuts silently do nothing.

Mouse support: click a tab name to switch to it directly. Within a tab, clicking an unfocused list
pane focuses it and highlights the clicked item — the mouse equivalent of pressing `<Alt-s>`/`<s>` —
and clicking an already-focused item confirms it, the same as arrow-keys followed by `<Enter>`. On
the System View tab, clicking the map enters map navigation. Mouse and keyboard can be freely mixed
at every step.

1. Open the **Research** tab and research an engine (e.g. Ion Drive) and a weapon (e.g. Ion Cannon)
2. Open the **Colonies** tab to grow your capital colony and queue up mines and factories — a ship needs both Engine Nozzles (via a Heat Resistant Alloy factory chain) and Microprocessors (via a Superconductors + Electronics chain) to build
3. Once research unlocks a module, open the **Ship modules** tab, select its type, then the module, and press Enter to finalize a design — you need both an engine and a weapon design
4. Press `<Alt-r>`/`<r>` in the Ship modules tab to build a ship from your current designs — press it again any time to build another and grow your fleet, even mid-fight
5. Your fleet starts at your home planet, while the enemy guards the outermost planet in the system — a ☠ marks the enemy and a ◆ marks your fleet on the System View map. Press `<Alt-e>`/`<e>` to send your fleet straight to the enemy, or `<Alt-c>`/`<c>` to send it straight home, without paging through the body list (you can still do that manually with `<Alt-s>`/`<s>` plus arrows, then `<Alt-r>`/`<r>`, to visit anywhere else). Travel takes time
6. Once your fleet reaches the enemy's location, combat resolves automatically each tick — every surviving ship fires at once, stacking damage, while the enemy's counter-fire always hits whichever ship is at the front of the fleet until it's destroyed. Sometimes a second, much tougher escort is guarding alongside the standard enemy — check the Shipyard panel's "Enemy fleet" line before you commit, since it keeps firing even while you're still working through the weaker one. Wipe out every enemy ship to win; if your whole fleet is destroyed first, build more ships and send them back to finish the job
7. Don't dawdle: if you never engage it, the enemy periodically advances toward your home planet (watch the **Fleet** panel in System View for a countdown) and, once it arrives, sieges your colony each tick until either you intercept it or the colony's HP hits 0 — losing the colony loses the game

## 🕹️ Example first session
A concrete, literal sequence of keypresses that gets a ship built and fighting from a fresh launch
(uses the plain-letter keys, which work in every terminal — see above):

Research and colony construction both tick in the background regardless of which tab you're looking
at, and multiple research items progress in parallel once started — so queue everything below
back-to-back rather than waiting for each step to finish before starting the next one:

1. Dismiss the splash screen: `Enter`
2. **Research tab** (`Tab` once): start both Ion Drive (engine) and Ion Cannon (weapon) right away —
   they research simultaneously, not one after another
   - `s`, `Down` (→ Sublight propulsion), `Enter`, `Enter` (→ Ion Drive, first in the list), `r`
   - `s`, `Down` ×6 (→ Space Warfare), `Enter`, `Enter` (→ Ion Cannon), `r`
3. **Colonies tab** (`Tab`): immediately select your colony and queue every building a ship needs —
   don't wait for research to finish first
   - `s`, `Enter` — selects your only colony
   - `r`, `Enter` — queues a Mine (queue 2–3 total for faster, more reliable resource output — mining
     yields are randomized per-resource, and one mine can leave you waiting a while)
   - `r`, `Down` ×3, `Enter` — Electronics factory
   - `r`, `Down` ×5, `Enter` — Heat Resistant Alloy factory
   - `r`, `Down` ×6, `Enter` — Superconductors factory
   - `r`, `Down` ×10, `Enter` — Engine Nozzles factory
   - `r`, `Down` ×11, `Enter` — Microprocessors factory
   - Wait for research to finish and for **Engine Nozzles** and **Microprocessors** to each reach 5 in
     the colony's info panel — this is the slow part (building construction alone takes a few minutes
     at the default speed), so this is a good point to explore the other tabs while it runs
4. **Ship modules tab** (`Tab`): design and build your first ship
   - `s`, `Enter`, `Enter` — confirms the Ion drive engine design
   - `s`, `Down`, `Enter`, `Enter` — confirms the Ion Cannon weapon design
   - `r` — builds a ship (press again any time to build more and grow your fleet)
5. **System View tab** (`Tab`): send your fleet to fight
   - `e` — sends your fleet straight to the enemy; combat then resolves automatically each tick

If your ship is destroyed before the enemy is, go back to step 4 and build another — the enemy keeps
whatever damage you've already dealt, so a second ship picks up where the first left off.

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
