/** Markov chain — JavaScript implementation. Pure functions, no state. */
const Mark = {
  build(notes) {
    const table = {};
    for (let i = 0; i < notes.length - 1; i++) {
      const from = notes[i];
      const to = notes[i + 1];
      table[from] = table[from] || {};
      table[from][to] = (table[from][to] || 0) + 1;
    }
    // Normalize
    for (const from of Object.keys(table)) {
      const total = Object.values(table[from]).reduce((a, b) => a + b, 0);
      for (const to of Object.keys(table[from])) {
        table[from][to] /= total;
      }
    }
    return table;
  },

  generate(table, start, length) {
    const seq = [start];
    let current = start;
    for (let i = 0; i < length - 1; i++) {
      const targets = table[current];
      if (targets) {
        const r = Math.random();
        let cumulative = 0;
        for (const [next, prob] of Object.entries(targets)) {
          cumulative += prob;
          if (r <= cumulative) {
            seq.push(Number(next));
            current = Number(next);
            break;
          }
        }
      } else {
        seq.push(current);
      }
    }
    return seq;
  }
};

if (typeof module !== 'undefined') module.exports = Mark;
if (require.main === module) {
  const training = [60, 62, 64, 65, 67, 65, 64, 62];
  const table = Mark.build(training);
  console.log('Training:', training);
  console.log('Generated:', Mark.generate(table, 60, 16));
  console.log('Table:', JSON.stringify(table, null, 2));
}
