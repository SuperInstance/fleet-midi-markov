# 🕸️ markov

**Feed it 8 notes. Get infinite variations. No GPU required.**

---

## Wait, show me

```python
from lib.markov import build_transition_table, generate

training = [60, 62, 64, 65, 67, 65, 64, 62]  # C major scale
probs = build_transition_table(training)
result = generate(probs, 32)
# → [60, 62, 64, 62, 64, 65, 67, 65, 67, 65, 67, 65, ...]
```

Eight notes of a C major scale produce an infinite stylized stream. The model learns that 64 (E) goes to 65 (F) 50% of the time and back to 62 (D) 50% of the time. That's the entire "training loop."

---

## No neural network. No GPUs

Just probability tables:

| Current | Next | Probability |
|---------|------|------------|
| 60 (C) | 62 (D) | 100% |
| 62 (D) | 64 (E) | 100% |
| 64 (E) | 65 (F) | 50% |
| 64 (E) | 62 (D) | 50% |

---

## 8 training sets → 8 unique styles

| Training | Character | Generated behavior |
|----------|-----------|-------------------|
| C major scale [60,62,64,65,67,65,64,62] | Stepwise | Stabilizes on 64-65-67 patterns |
| Arpeggio [60,64,67,72,76,72,67,64] | Leaping | Octave jumps, open voicings |
| Chromatic [67,66,65,64,64,65,66,67] | Tight | Hovering around a small range |
| Bass [57,60,64,57,60,64,57,60,64,57] | Repetitive | Rock-solid repeating patterns |
| Wide intervals [60,67,64,72,67,76,72,81] | Exploratory | Big leaps, then graceful returns |

**Ensign Weaver** — Fleet Statistical Music Officer.
