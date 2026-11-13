# PC-to-Android Wi-Fi Speaker

A Windows PC program (Rust) captures the PC's system audio and streams it over Wi-Fi to an Android app (Kotlin), which plays it through the phone's speaker, so the phone works as a wireless speaker for the PC.

Full project plan (claude.ai doc): https://claude.ai/artifact/BUTEFv7nyHdB14u4eQGGw9

## About me and how to work with me

- **I'm a beginner in both Rust and Kotlin.** I have never written code in either language.
- **Teach me step by step.** Don't write the whole project for me. For each step:
  1. Explain the concept in plain words (what it is, and why we need it here).
  2. Show a small amount of code and explain the new parts line by line.
  3. Let me type it and run it myself. Wait for me to report the result before moving on.
  4. When I get an error, help me read and understand it, not just fix it.
- Keep each step small: one new concept at a time, ending with something I can run.
- Introduce Rust and Kotlin concepts when the project needs them, not as a separate course.
- Check my understanding now and then with a quick question.

## My setup

- **PC:** Windows
- **Rust:** installed with rustup; editor is Zed (rust-analyzer is built in)
- **Kotlin/Android:** Android Studio, SDK Android 16 "Baklava" (API 36.1)
- **Phone:** Android, connected over the same Wi-Fi network as the PC (USB only for installing/debugging the app)

## Current status

- [x] Rust installed (rustc/cargo 1.100.0, rustup 1.29.3)
- [x] Zed editor (rust-analyzer built in)
- [x] Android Studio installed and SDK downloaded (API 36.1)
- [ ] Phone: Developer options and USB debugging enabled
- [x] Phase 0 started: `pc/` cargo project (pc-speaker) runs Hello, world!
- [x] Git initialized at repo root (working branch: `develop`)

Next step: add a `.gitignore` (ignore `pc/target/`), make the first commit, then continue Rust basics.

## Design decisions (already agreed)

- **Transport: Wi-Fi, UDP.** Not USB, not Bluetooth (most Android phones can't act as a Bluetooth A2DP sink). UDP for audio because TCP retransmits stall playback on lossy Wi-Fi. A TCP control channel is optional later.
- **Capture:** WASAPI loopback on Windows, via the `cpal` crate (build an input stream on the default *output* device). Fallback: the `wasapi` crate.
- **Playback:** Android `AudioTrack` in streaming mode.
- **Compression:** start with raw PCM; Opus is an optional later phase.
- **No network or blocking work in the audio callback** (Rust) or on the UI thread (Android).

## Audio format (fixed)

- 48,000 Hz, signed 16-bit, stereo, little-endian PCM
- One frame = 10 ms = 480 samples per channel = 1,920 bytes
- Raw PCM bandwidth is about 1.5 Mbps

## Packet format (from Phase 4)

12-byte header followed by the audio payload. Header numbers are big-endian; PCM samples stay little-endian.

| Bytes | Field     | Meaning                                   |
|-------|-----------|-------------------------------------------|
| 0–1   | Magic     | ASCII `AS`                                |
| 2     | Version   | `1`                                       |
| 3     | Codec     | `0` = raw PCM, `1` = Opus                 |
| 4–7   | Sequence  | u32, +1 per packet                        |
| 8–11  | Timestamp | u32 sample position, +480 per packet      |
| 12…   | Payload   | Audio data                                |

Ports: **50505** UDP audio, **50506** UDP discovery.

## Repository layout

```
/
├── CLAUDE.md
├── pc/        # Rust sender (cargo project: pc-speaker)
└── android/   # Kotlin Android app (Android Studio project)
```

Use Git from the start; commit each time a milestone works.

## Phases

0. **Tools and basics.** Install everything; Rust basics (Rust book ch. 1–9, Rustlings); Kotlin basics and Android Basics with Compose. Milestone: hello-world in both.
1. **Android receiver.** INTERNET permission; coroutine on `Dispatchers.IO` with a `DatagramSocket(50505)` writing packets to `AudioTrack`; show the phone's IP; Start/Stop buttons. No header yet.
2. **Rust test-tone sender.** `UdpSocket` sends a 440 Hz sine as 1,920-byte chunks every 10 ms, paced by `Instant` so timing errors don't add up. Milestone: steady beep from the phone.
3. **Windows loopback capture.** `cpal`; convert f32 → i16; set Windows output to 48 kHz (or resample with `rubato`); callback pushes into a channel/ring buffer, a separate thread chunks and sends. Milestone: PC audio plays on the phone.
4. **Header + jitter buffer.** Add the header. Phone: receiver coroutine fills a map keyed by sequence; player coroutine starts after ~50 ms buffered and plays in order every 10 ms; missing packet → silence or repeat; late → drop; buffer > ~150 ms → drop oldest. Adjustable target buffer and on-screen stats. Milestone: smooth, ~60–100 ms delay.
5. **Opus (optional).** Rust `opus` crate (needs CMake), 128 kbps, 10 ms frames, codec byte `1`. Android: `MediaCodec` `audio/opus`, or libopus via NDK for packet-loss concealment.
6. **Polish.** PC broadcasts on 50506, phone lists PCs and replies "send to me" (hold a `MulticastLock`); foreground service of type `mediaPlayback` (+ `FOREGROUND_SERVICE_MEDIA_PLAYBACK`); `WifiLock` with `WIFI_MODE_FULL_LOW_LATENCY`; tray icon (`tray-icon`) or `egui` GUI; `cargo build --release`.

## Known pitfalls

- Windows Firewall can silently block UDP; allow the program on private networks.
- PC and phone must be on the same network; guest networks or AP isolation block it.
- Prefer 5 GHz Wi-Fi.
- Mixing up byte order gives noise or wrong sequence numbers.
- Early on, using `.clone()` freely to satisfy the borrow checker is fine.
