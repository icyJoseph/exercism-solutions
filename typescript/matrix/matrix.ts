type Pattern = string;
type Input = string;
type Dimension = number[];
type MultiDimension = Dimension[];

const $split = (pattern: Pattern) => (src: Input) => src.split(pattern);
const $parse = (str: string): number => parseInt(str);
const $vecString2Num = (vec: string[]) =>
  vec.map(dimension => $parse(dimension));

const transpose = (matrix: MultiDimension) =>
  matrix.reduce(
    (transposed: MultiDimension, dimension: Dimension) =>
      transposed.map((ortogonal, index) => [...ortogonal, dimension[index]]),
    [[], [], []]
  );

class Matrix {
  matrix: Input;
  rows: MultiDimension;
  columns: MultiDimension;

  constructor(matrix: Input) {
    this.matrix = matrix;
    this.rows = this.getRows();
    this.columns = this.getCols();
  }

  getRows(): MultiDimension {
    return this.matrix
      .split("\n")
      .map($split(" "))
      .map($vecString2Num);
  }

  getCols(): MultiDimension {
    return transpose(this.getRows());
  }
}

export default Matrix;
