# MYgene Enter-to-chat Windows client

This branch is a **client-only** experiment based on the Rustnarok fork. It does not change rAthena, accounts, rates, or Codespaces networking.

## Controls

- On launch: chat input inactive; a faint translucent strip replaces the solid white input.
- Enter (inactive): activate the chat input.
- Enter (active, with text): send/execute the message and return to inactive mode when the draft is consumed.
- Enter (active, empty): return to inactive mode without sending.
- Esc (active): cancel the unsent draft and return to inactive mode.
- Clicking the faint message input: activate typing.
- Chat messages, chat history, and existing shortcut behavior remain available.
- The existing /bm or chat-mode option may temporarily disable this behavior. MYgene stores this preference in a separate `mygene_chatmode` setting so it does not alter the standard client's `chatmode` preference.

This remains an **experimental fork**, not an official Rustnarok release. No modified files from protected Ragnarok clients are included.

## Windows build through GitHub Actions

Open **Actions → MYgene Windows Client → Run workflow**, select `mygene-enter-chat`, and run it. A push to this branch also triggers the workflow. It:
1. Executes `cargo test --locked`.
2. Builds `cargo build --locked --release --bin rustnarok-client` on a Windows runner.
3. Uploads a ZIP artifact named `MYgene-Rustnarok-Windows` containing the executable and its SHA-256 checksum.

**A workflow passing is required before calling the build verified.** The client must still be tested in the game for chat UX and keyboard shortcuts.

## Test without overwriting your working installation

1. Keep the original Rustnarok v0.4.0 executable and your working `data.grf`, `data/clientinfo.xml`, and authorized assets.
2. Extract the CI artifact ZIP to a separate folder.
3. Rename the new `rustnarok-client.exe` to `rustnarok-client-mygene.exe`, then copy that EXE next to your existing Rustnarok executable. Do not overwrite it.
4. Continue using your already-working private GitHub Codespaces port forwards. Run the renamed EXE from the game folder (PowerShell: `./rustnarok-client-mygene.exe`).
5. Check Enter / type / Enter; empty Enter; Esc cancel; click to activate; chat history visibility; that F-keys and existing gameplay controls still function when input is inactive. If errors occur, keep using the unchanged original EXE.

Windows Defender should remain enabled. Do not bypass any detected threat. No passwords, personal GRFs, or database exports should be committed to this repository.

## Manual build

On Windows with a supported stable Rust toolchain:

```powershell
cargo test --locked
cargo build --release --locked --bin rustnarok-client
```

The result is `target/release/rustnarok-client.exe`. Only use it with game data you are authorized to use.
