export function List(initial = []) {
  this.values = [];
  initial.forEach(el => this.values.push(el.values || el));
  return this;
}

List.prototype.append = function(list) {
  this.values.push(...list.values);
  return this;
};

List.prototype.concat = function(list) {
  list.values.forEach(el => this.values.push(...el));
  return this;
};

List.prototype.filter = function(fn) {
  let arr = [];
  this.values.forEach(el => {
    if (fn(el)) {
      arr.push(el);
    }
  });
  return new List(arr);
};

List.prototype.length = function() {
  let length = 0;
  this.values.forEach(_ => (length = length + 1));
  return length;
};

List.prototype.map = function(fn) {
  let arr = [];
  this.values.forEach(el => arr.push(fn(el)));
  return new List(arr);
};

List.prototype.foldl = function(fn, start) {
  let acc = start;
  this.values.forEach(el => (acc = fn(acc, el)));
  return acc;
};

List.prototype.foldr = function(fn, start) {
  let acc = start;
  const maxIndex = this.length() - 1;
  this.values.forEach(
    (_, index) => (acc = fn(acc, this.values[maxIndex - index]))
  );
  return acc;
};

List.prototype.reverse = function() {
  let _reversed = [];
  this.values.forEach(el => (_reversed = [el, ..._reversed]));
  this.values = _reversed;
  return this;
};
