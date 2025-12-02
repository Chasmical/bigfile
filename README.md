# 🤍 bigfile
**bigfile** is a tool for viewing *bigfiles*—files used for storing game assets in KarmaZoo (and possibly other games using the same engine, though I'm not aware of any)
![](assets/screenshot.png)

## Usage
1. Click File —> Open or press <kbd>Ctrl</kbd> + <kbd>O</kbd>
2. Select bigfiles in order: `bigfile.bfn` —> `bigfile.bfdb` —> `bigfile.bfdata`
> [!TIP]
> For KarmaZoo, these are located in `%KarmaZoo%/resources/cookedData`, where `%KarmaZoo%` is your game installation directory
3. Extract selected files or extract all the files

## Building
This project uses [Just](https://just.systems) to run building and bundling commands.

- `just` — runs the app (does `just run` under the hood)
- `just run` — builds and runs the app with optional parameters
- `just build` — builds the lib and the app with optional parameters
- `just bundle` — bundles the release version of the app. On Windows, it just moves the executable file to `build/`. On macOS, it also makes it an app bundle with icons

Alternatively you can just use `cargo` for building and running.

## Attributions
- [Twemoji](https://github.com/twitter/twemoji) by Twitter, licensed under CC-BY 4.0. See [assets/ATTRIBUTION](assets/ATTRIBUTION)
