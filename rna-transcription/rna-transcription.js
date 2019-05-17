const transcribe = nucleotide => {
  switch (nucleotide) {
    case "C":
      return "G";
    case "G":
      return "C";
    case "T":
      return "A";
    case "A":
      return "U";
    default:
      return nucleotide;
  }
};

export const toRna = sequence =>
  sequence
    .split("")
    .reduce((acc, current) => `${acc}${transcribe(current)}`, "");
