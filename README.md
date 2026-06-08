<div align="center">

# 🕸️ fleet-midi-markov

> *Statistical MIDI generation without neural networks*

[![CI](https://img.shields.io/github/actions/workflow/status/SuperInstance/fleet-midi-markov/ci.yml?style=flat-square&logo=github&label=CI)](https://github.com/SuperInstance/fleet-midi-markov/actions)
[![npm](https://img.shields.io/badge/npm-%40superinstance%2Fmidi--markov-cb3837?style=flat-square&logo=npm)](https://www.npmjs.com/package/@superinstance/midi-markov)
[![Docker](https://img.shields.io/badge/docker-ghcr-2496ed?style=flat-square&logo=docker)](https://github.com/SuperInstance/fleet-midi-markov/pkgs/container/fleet-midi-markov)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen?style=flat-square)](http://makeapullrequest.com)

---

Builds probability transition matrices from any MIDI training file and generates infinite stylized sequences. Verified: 8 training notes produce a walking bass: [60,62,64,62,64,65,64,65,...]. No GPU required.

---

## 📦 Installation

```bash
# npm
npm install @superinstance/midi-markov

# Docker
docker pull ghcr.io/superinstance/fleet-midi-markov:latest

# Clone
git clone https://github.com/SuperInstance/fleet-midi-markov.git
```

## 🚀 Quick Start

```bash
# Train and generate from 8 training notes:
python3 -c "
from lib.markov import build_transition_table, generate
training = [60, 62, 64, 65, 67, 65, 64, 62]
probs = build_transition_table(training)
print(f\"Generated: {generate(probs, 16)}\")
"
```

## 🏗️ Architecture

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│   Training: [60, 62, 64, 65, 67, 65, 64, 62]            │
│         │                                                │
│         ▼                                                │
│   ┌────────────────────┐                                  │
│   │ Transition Table   │  60 → 62: 100%                   │
│   │ Markov Chain       │  62 → 64: 100%                   │
│   │ P(next|current)    │  64 → 65:  50%, 64 → 62: 50%     │
│   └────────┬───────────┘  65 → 67:  50%, 65 → 64: 50%     │
│            ▼                                              │
│   ┌────────────────────┐                                  │
│   │ Generate           │ → [60,62,64,62,64,65,64,65,...] │
│   │ Infinite sequence  │ → Infinite jazz from 8 notes     │
│   └────────────────────┘                                  │
│                                                          │
│   No neural network. No GPU. Just probability.           │
│   Feed it any MIDI → it learns that style instantly.     │
└──────────────────────────────────────────────────────────┘
```

## 📡 API

### `build_transition_table(notes)` → probability dict
Analyzes note transitions in a training sequence.

### `generate(probs, length=16, start=None)` → note list
Generates an infinite-length sequence from the transition probabilities.

## 🧪 Beta Tested

Part of the [SuperInstance MIDI Fleet](https://github.com/SuperInstance/construct-coordination/blob/main/FLEET_MIDI.md). Every push verified via CI — zeroshot tests ensure zero-config operation out of the box.

## 🤝 Related

- [fleet-bridge](https://github.com/SuperInstance/fleet-bridge) — I2I bottle transport
- [construct-coordination](https://github.com/SuperInstance/construct-coordination) — Fleet catalog

---

<div align="center">
<sub>Built with 🕸️ for the SuperInstance fleet • <a href="https://github.com/SuperInstance">github.com/SuperInstance</a></sub>
</div>
