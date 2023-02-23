# Leaf for Visual Studio Code

This is the **official extension** for the [Leaf Programming Language][homepage] within [Visual Studio Code][vscode].

As the language is under active development, this extension is also under active development, and as such is considered to be **unstable**. If you find any bugs or have any suggestions, please open an issue on the [GitHub repository][issues].

## Getting Started

<!-- To get started, you'll need to install the extension from the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=leaf-language.leaf). <-- this once it is more stable -->

Make sure you have the [`vsce`][vsce] tool installed. If you don't, run the following command using your preferred package manager:

```bash
pnpm add -g vsce
```

With `vsce` installed, from the `editors/vscode` directory you can run the following commands to install and package the extension:

Install the extension locally:

```bash
pnpm install # <-- or `npm install` or `yarn install`
```

Package the extension:

```bash
vsce package
```

Then install VSCode by switching to the Extensions section on the left, then the `...` at the top middle and choose "Install from VSIX..." and choose the package you just created.

In the settings for `leaf` under **Preferences: Open Settings (UI)** from the command palate (`cmd/ctrl + shift + p`), set the **Leaf Compiler: Executable Path** to your configured `/path/to/bin/leafc`. This must be an absolute path, you cannot use `$LEAFC_INSTALL` here.

Alternatively, modify the ``.vscode/settings.json`` file to have an entry similar to the following:

```
"leafLanguageServer.compiler.executablePath": "/path/to/leaf-install/bin/leafc"
```

<!-- ## Features

Describe specific features of your extension including screenshots of your extension in action. Image paths are relative to this README file.

For example, if there is an image subfolder under your extension project workspace:

\!\[feature X\]\(images/feature-x.png\)

> Tip: Many popular extensions utilize animations. This is an excellent way to show off your extension! We recommend short, focused animations that are easy to follow.

## Requirements

If you have any requirements or dependencies, add a section describing those and how to install and configure them.

## Extension Settings

Include if your extension adds any VS Code settings through the `contributes.configuration` extension point.

For example:

This extension contributes the following settings:

* `myExtension.enable`: Enable/disable this extension.
* `myExtension.thing`: Set to `blah` to do something.

## Known Issues

Calling out known issues can help limit users opening duplicate issues against your extension.

## Release Notes

Users appreciate release notes as you update your extension.

### 1.0.0

The initial release of ...

### 1.0.1

Fixed issue #.

### 1.1.0

Added features X, Y, and Z. -->

[homepage]: https://leaf.dev
[vscode]: https://code.visualstudio.com/
[issues]: https://github.com/pulanski/leafc/issues
[vsce]: https://code.visualstudio.com/api/working-with-extensions/publishing-extension#vsce
