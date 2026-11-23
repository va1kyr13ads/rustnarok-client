# MYgene Blue Chon Chon Card — approved illustration

**Card item ID:** `60011`  
**Original artwork family:** Chonchon Card (ID `4009`)  
**Status:** Client feature branch recognizes the custom illustration; deploy the approved BMP alongside the matching Windows client.

## Installation

Download the approved artwork supplied with this change, named `mygene_blue_chon_chon_card.bmp` (300 × 400 px, 24-bit RGB BMP). In the folder containing `MYgeneClient.exe`, create the directories below if necessary and place it at:

```text
data/texture/유저인터페이스/cardbmp/mygene_blue_chon_chon_card.bmp
```

The artwork is a blue recolor of the normal Chonchon card's full illustration. It contains **no CHONCHON title**. The original card style, face, wings, forest and musical note are kept. Screenshot-only floating controls were removed from the supplied image.

The client's `CardPictures` maps item `60011` to `mygene_blue_chon_chon_card` when that BMP exists beside the executable, under `RUSTNAROK_CLIENT_DATA`, or in the authorized player's GRF. It loads through the same card View button as official cards. If the custom BMP is missing, the client falls back safely to Chonchon Card `4009` artwork without losing the card information. The standard card icon/collection art still inherits item `4009`.

**Expected SHA-256 (approved 300×400 BMP):**
`84148f676ba4bea40aba7a7880cf49f9fd1cea9fad7a23fb76266ca103cf26fa`

## Test on Windows

1. Use a build of the matching feature branch `feature/blue-chon-chon-slayer-20261123`.
2. Extract the artwork into the same directory tree as `MYgeneClient.exe` and restart the client.
3. On a **disposable development character**, give a GM-test Blue Chon Chon Card with `@item 60011 1`.
4. Open the card's item description, then use **View** to see the blue, nameless full illustration.
5. Remove the BMP and relaunch to confirm the original Chonchon card illustration appears as a fallback; restore the BMP afterward.
6. Confirm the inventory icon and collection thumbnail are still recognizable; 90% bow execution remains server-controlled and is not affected by art loading.

The repository does not redistribute a proprietary Ragnarok GRF. Do not deploy the unmerged matching server source onto a populated world before the existing Slayer ID migration gates pass.
