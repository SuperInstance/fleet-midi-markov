"""Markov Chain MIDI generator — analyze MIDI, generate infinite stylized streams."""
import random
from collections import defaultdict

def build_transition_table(notes):
    transitions = defaultdict(lambda: defaultdict(int))
    for i in range(len(notes) - 1):
        transitions[notes[i]][notes[i+1]] += 1
    probs = {}
    for note, nexts in transitions.items():
        total = sum(nexts.values())
        probs[note] = {n: c/total for n, c in nexts.items()}
    return probs

def generate(probs, length=16, start=None):
    if start is None:
        start = list(probs.keys())[0] if probs else 60
    seq = [start]
    for _ in range(length - 1):
        curr = seq[-1]
        if curr not in probs:
            seq.append(60)
        else:
            r = random.random()
            cumulative = 0
            for note, prob in probs[curr].items():
                cumulative += prob
                if r <= cumulative:
                    seq.append(note)
                    break
            else:
                seq.append(60)
    return seq

if __name__ == '__main__':
    training = [60, 62, 64, 65, 67, 65, 64, 62]
    probs = build_transition_table(training)
    print(f"Generated sequence: {generate(probs, 12)}")
