# Getting Started

[Makepad](https://github.com/makepad/makepad) is a cross-platform UI framework written in Rust.
It is in active development, but is already usable to build quick prototypes and simple (or even complicated UI) applications.

One of the key features of the Makepad is its ability to simply, and quickly, build and run applications on multiple platforms, including MacOS, Windows, Linux, Android, iOS, and WebAssembly.

Here are the current instructions on how to build and run Makepad applications on the different platforms.

## Pre-requisites

If haven't yet, please setup the Rust development environment by following the [official Rust setup instructions](https://www.rust-lang.org/tools/install).

### Assumption

For the quick start, we will use the "simple" example app from the official Makepad project

Name of application: `makepad-example-simple`

The same steps can be adapted to any one of the existing example apps in the [*Makepad* examples folder](https://github.com/makepad/makepad/tree/master/examples).

## Build & Run Instructions

Follow step 1 commands below for initial setup of the Makepad build and run environment.
After step 2, you may choose any one or more of the platforms you're interested in building for.

## 1. Setup Makepad

Replace `projects` with your own directory name.

```bash
cd ~/projects
```

### Clone the Makepad repository

```bash
git clone https://github.com/makepad/makepad.git
```

or

```bash
git clone git@github.com:makepad/makepad.git
```

### Change to latest 'rik' branch (Recommended)

```bash
cd ~/projects/makepad
git branch rik
```

### Install makepad subcommand for cargo

```bash
cd ~/projects/makepad
cargo install --path ./tools/cargo_makepad
```

### Install platform toolchains

```bash
rustup toolchain install nightly
```

## 2. Go To Examples folder (Optional)

```bash
cd ~/projects/makepad/examples
ls -l
```

Your system is now setup to build and run Makepad example applications. All examples in this directory have the application name of `makepad-example-` prefix plus the name of directory.

## To Build and Run on macOS or PC

Running on Desktop is the quickest way to try out an example app.

```bash
cd ~/projects/makepad/examples/simple
cargo run
```

or

```bash
cd ~/projects/makepad
cargo run -p makepad-example-simple
```

And there should be a desktop application window now running (may need to click on the icon on MacOS's Dock to show it)

## To Build and Run on Android Emulator or Device

### Install Android toolchain (First time)

```bash
cargo makepad android install-toolchain
```

### Install app on Android device or Android emulator

Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a single device connected properly, then install and run as below:

```bash
cd ~/projects/makepad
cargo makepad android run -p makepad-example-simple --release
```

The application will be installed and launch on either the emulator or device.

## To Build and Run on Apple iOS Simulator or Device

### Install IOS toolchain (First time)

```bash
xcode-select --install
cargo makepad apple ios install-toolchain
```

### Install app on Apple devivce or iOS Simulator

### iOS Setup

For iOS, the process is slightly more complicated. The steps involved are:

1. Enable your iPhone's Developer Mode, please see instructions here: [Enable Developer Mode](https://www.delasign.com/blog/how-to-turn-on-developer-mode-on-an-iphone/)
1. Setup an Apple Developer account
1. Setup an empty skeleton project in XCode
    1. File -> New -> Project to create a new "App"
    1. Set the Product Name as **`makepad-example-simple`** (used in --app later)
    1. Set the Organization Identifier to a value of your choice, for this example we will use **`my.test`** (used in --org later)
    1. Setup the Project Signing & Capabilities to select the proper team account
1. In XCode, Build/Run this project to install and run the app on the simulator and device
1. Once the simulator and device has the "skeleton" app installed and running properly, then it is ready for Makepad to install its application.

### Makepad Install

We will run the `cargo makepad apple ios` command, similar to Android build above, but there are some 2 to 4 additional parameters that need to be filled in:

**`--org`**

First few parts of the organization identifier (which makes up the Bundle Identifier). Usually in the form of **com.somecompany** or **org.someorg**, etc.
This is the same value used to setup the initial skeleton app above. For this example:
> `my.test`

**`--app`**

The name of the application or the project. This is the same as the Product Name used to setup the initial skeleton app above. In this case:
> `makepad-example-simple`

**`--org-id`** (real-device only)

Find the `<key>ApplicationIdentifierPrefix</key>` and use the value in the `<string>ORGIDVALUE</string>`, from the newest `**.mobileprovision` file located in the `~/Library/MobileDevice/Provisioning Profiles` directory.
`ORGIDVALUE` should be a 10 digit alpha-numeric value.

**`--ios-version`** (OPTIONAL)

defaults to 17. Set it to 16 or other values if the device is not running iOS 17.

### Example

For this example, we have the Bundle Identifier of **`my.test.makepad-example-simple`**

### Install app on IOS simulator

```bash
cd ~/projects/makepad
cargo makepad apple ios --org=my.test --app=makepad-example-simple run-sim -p makepad-example-simple --release
```

### Install app on IOS device

```bash
cd ~/projects/makepad
cargo makepad apple ios --org-id=ORGIDVALUE --org=my.test --app=makepad-example-simple run-device -p makepad-example-simple --release
```

The application will be installed and launched on either the emulator or real device.

## To Build and Run on Web/Browser via WASM

*Coming Soon*
