const nucleotides = ["A", "C", "G", "T"];

const initialValue = nucleotides.reduce(
  (prev, nucleotide) => ({ ...prev, [nucleotide]: 0 }),
  { error: false }
);

export const NucleotideCounts = {
  // expose a function to parse dna
  parse(dna) {
    // reduce over the dna
    const { error, ...parsed } = dna.split("").reduce(
      ({ error, ...rest }, nucleotide) => ({
        ...rest,
        [nucleotide]: rest[nucleotide] + 1,
        error: error || !nucleotides.includes(nucleotide)
      }),
      initialValue
    );

    // if we found an error during the reduce
    // throw
    if (error) {
      throw Error("Invalid nucleotide in strand");
    }

    // otherwise just print
    return Object.values(parsed).join(" ");
  }
};
