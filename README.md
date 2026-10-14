# Stardew Predictor

A tool to predict events in Stardew Valley based on the seed (and other information). Use it [here](https://mrkinghatter.github.io/stardew-predictor/).

This is a fork of [Oshawk's Stardew Predictor](https://github.com/Oshawk/stardew-predictor) that adds Stardew Valley **1.6** support (Switch and PC), building on the original 1.5.6 predictor.

## Supported Platforms

- PC
- Switch

The platform matters: Switch uses the Mono JKISS generator while PC uses .NET's generator, so predictions are computed per platform.

## Supported Versions

- 1.6 (cart, Krobus, Sandy, Pierre, Joja)
- 1.5.6 (original Oshawk predictor)

Select the game version and enter your game ID (the cracked seed). 1.6 geode prediction is not available because it needs the player's unique multiplayer ID, which a cracked game ID does not provide.

## Credits

- Original predictor and WASM/Yew app by [Oshawk](https://github.com/Oshawk/stardew-predictor).
- 1.6 RNG and shop logic verified against [MouseyPounds' Stardew Predictor](https://github.com/MouseyPounds/stardew-predictor).

## Licences

Stardew Predictor's code is licenced under MIT.

The sprites and data located in the `assets` directory were extracted from Stardew Valley using [xnbcli](https://github.com/LeonBlade/xnbcli). Their copyright is held by ConcernedApe.