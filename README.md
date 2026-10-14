# AirCard (Windows) 🎴

> **Apple Wallet Card Skinner & Lockscreen Passcode Themer for iOS 18+ (No Jailbreak Required)**  
> Native Windows client written in Rust. Powered by the `airlift` AirTraffic sync exploit.

---

## Compatibility

| iOS Version | Status | Notes |
| :--- | :--- | :--- |
| **iOS 18.0 – 27.0.1** | ✅ Supported | Full support for Wallet skins and Lockscreen passcode themes |
| **iOS 27.2 beta 1 – beta 2** | ✅ Supported | Working |
| **iOS 27.2 beta 3+** | ❌ Patched | Apple patched the underlying `airlift` exploit. Flashing will not work. |

---

> [!IMPORTANT]
> Do not update to **iOS 27.2 beta 3 or newer** if you want to continue using AirCard. The underlying AirTraffic sync exploit was patched by Apple in beta 3.
> 


## Features
- 🎨 **Custom Card Skins:** Assign custom artwork, textures, or bank logos to Apple Pay and Apple Cash cards.
- 🔢 **Lock Screen Passcode Themes (.passthm):** Apply custom keypad button artwork from popular Cowabunga & Nugget `.passthm` themes directly to iOS lockscreen.
- ⚡ **100% Native & Lightweight:** Single standalone `aircard.exe` (~7.5 MB). No Python, no Flet, no webview, no bloated runtimes.
- 🪟 **Material Design 3 Interface:** Clean, modern dark theme built with `egui` and `eframe`.
- 📱 **Zero-Hassle Card Detection:** Tap any card in your iPhone's Wallet app while connected to detect its hash in real-time via `syslog_relay`.
- 🩺 **Environment Check:** Diagnose Apple components, service status, Lockdown access, selected transport, and native iPhone pairing with guided setup steps.
- 📶 **USB & WiFi Transport:** Scan card events and apply Wallet or passcode assets through USB or a paired local WiFi connection.
- 🌐 **English / Simplified Chinese UI:** Switch the interface language from the status bar; the selection is saved locally for future launches.
- 🔄 **Safe & Reversible:** Complete Books state snapshot and automatic restore engine — preserves original device state and backs up the original Wallet card face before replacing it.
- 🚀 **Zero Jailbreak:** Utilizes Apple's built-in AirTraffic sync conduit without modifying system partitions or disabling security.

---

## Requirements
- **Windows 10 / 11 (64-bit)**
- **Apple Mobile Device Support / 64-bit iTunes** (required for Apple device communication).
- A Lightning or USB-C cable for the initial trust/pairing setup.
- For WiFi mode, enable WiFi sync and keep the PC and iPhone on the same local network.

---

## Environment setup & troubleshooting

Open **Environment** to run the checks. Missing Apple components open this page automatically on startup. Checks run in a separate process with a 20-second limit, so an unresponsive Apple API does not freeze the UI. Device operations are paused while the check is running.

The report verifies DLL loading and exports, Windows service status, the usbmux device channel, **ReadBUID through the Apple service**, the selected USB/WiFi connection, and native pairing/session authentication with a `com.apple.atc` service probe. BUID access can fail when the service account cannot access `%ProgramData%\Apple\Lockdown`, causing `0xE8000003` or an AirTraffic timeout even when the phone is listed. A passed service probe confirms prerequisites; it does not prove a completed artwork transfer.

Use the page's **Apple setup guide**, **Open Services**, folder shortcuts, and copyable service restart command to configure the PC. Connect the unlocked iPhone by USB and complete trust in iTunes, then close iTunes before applying artwork. **Copy report** includes check results and next steps without pairing certificates or private keys. Detection does not re-pair the phone, change permissions, or write artwork. After changing the selected device or connection mode, check again.

The page shows an animated progress ring, compact status summaries, and the next action to take. **Technical details** retains the original errors and all setup advice; skipped checks are marked **Not checked**.

**One-click setup** chooses a fix from the check results. Service repair starts or restarts Apple Mobile Device Service, enables automatic startup, and checks again. Component repair uses the installed Apple MSI packages; if components or repair media are missing, it downloads the official 64-bit iTunes installer and verifies its Apple signature before opening it. Windows requests administrator approval for these changes. After component repair, restart AirCard to reload its DLLs; a required Windows restart is shown explicitly. Connection setup opens iTunes and guides the user through USB trust on the phone. Repair results are included in the copied report. No pairing records are deleted and directory permissions are not weakened.

---

## Installation

### Pre-built Executable
1. Download **`aircard.exe`** from [Releases](https://github.com/Lumid-Off/AirCard-Windows/releases).
2. Connect your iPhone via USB, unlock it, and tap **"Trust this Computer"** if prompted.
3. Run **`aircard.exe`**. After WiFi sync is enabled, later sessions can work without the cable.

---

## WiFi Connection Setup
1. Connect the iPhone by USB for the initial pairing.
2. In Apple Devices or iTunes, enable **Show this iPhone when on Wi-Fi** / **Sync with this iPhone over Wi-Fi**.
3. Apply the setting, then keep the iPhone and PC on the same local network.
4. In AirCard, click **Refresh** and confirm the device shows a **WiFi** transport.
5. Disconnect the cable, click **Refresh** again, and select **WiFi only**. Use **Auto (USB preferred)** when automatic fallback is desired.

If both transports are available, **Auto** uses USB first and falls back to WiFi. For a guaranteed end-to-end WiFi route, disconnect the USB cable, click **Refresh**, and then choose **WiFi only**. This is required because Apple's AirTraffic API selects its route by UDID rather than accepting a transport parameter.

---

## How to Customize Apple Wallet Cards
1. Connect your iPhone through USB or paired WiFi and ensure it is unlocked.
2. In AirCard, stay on the **Wallet** tab and click **Scan**.
3. On your iPhone:
   - Open **Apple Wallet** (or double-click the Side/Power button).
   - Tap the card you want to customize.
   - AirCard intercepts and saves the card hash automatically. Click **Stop**.
4. Click **Choose Image...** to pick your artwork (PNG, JPG, or WebP — drag inside the preview to position the crop, then scale it to `1536 × 969`).
5. Click **Apply Card Skin**.
6. Force-close the **Wallet** app on your iPhone from the App Switcher (swipe up from bottom, then swipe Wallet away) and reopen Wallet to see your new card!
7. The first apply stores a local backup of the original card face. Use **Restore Original** later to write it back and invalidate Wallet's cached artwork.

---

## How to Apply Lockscreen Passcode Themes (.passthm)
1. Switch to the **Passcode** tab in AirCard.
2. Click **Choose .passthm...** and select any `.passthm` package (Cowabunga or Nugget).
3. Select your target iOS version cache:
   - **Auto (TelephonyUI-10)** — iOS 18+ (Default)
   - **TelephonyUI-9** — iOS 16 - 17
   - **TelephonyUI-8** — Legacy iOS
4. Click **Apply Passcode Theme**.
5. Lock your iPhone screen or open Phone dialer to see your new custom passcode keypad buttons!

> [!IMPORTANT]
> **Turn OFF Bold Text:**  
> On your iPhone, go to **Settings ➔ Display & Brightness** and make sure **Bold Text** is turned **OFF**. If Bold Text is enabled, iOS ignores cached dialer button graphics and renders system vector fonts instead.

---

## Building from Source

Prerequisites: [Rust toolchain](https://rustup.rs/) (`stable-x86_64-pc-windows-msvc`).

```powershell
# Clone the repository
git clone https://github.com/Lumid-Off/AirCard-Windows.git
cd AirCard-Windows

# Run tests
cargo test

# Build release binary
cargo build --release
```

The compiled binary will be in `target\release\aircard.exe`.

---

## Contributors
- **[@Lumid-Off](https://github.com/Lumid-Off)** (Windows Native Rust Port & Maintainer) — [GitHub](https://github.com/Lumid-Off) · [Twitter / X](https://x.com/LumidOff)
- **[@mak5er](https://github.com/mak5er)** (Original macOS App & Exploit Research) — [GitHub](https://github.com/mak5er) · [Twitter / X](https://x.com/mak5er)
- **[AirLift](https://github.com/0xjohnnydev/airlift)** by **[0xjohnny (@0xjohnnydev)](https://github.com/0xjohnnydev)**: Original AirTraffic/ATAirlock sandbox escape and proof of concept underlying `AirliftFFI`.

## Credits
- Core exploit based on `airlift` (AirTraffic sync escape).
- Theme format inspired by [Cowabunga](https://github.com/leminlimez/Cowabunga) and [Nugget](https://github.com/leminlimez/Nugget).

## Support

If you find AirCard useful, you can support future development:

- **TON**: `UQB5jbOhep98IvgKjCIsJ1hHGRh2iWkacAZh9jW2DMIVSWZm`
- **USDT (TRC20)**: `TLiVnkPZ7mVKwCD9RfE28uVTFk6sD6rux2`
- **USDT / BNB (BEP20)**: `0x8EA94e79e47FafBCE10E65342D875c86d1019541`

