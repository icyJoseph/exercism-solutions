export function InputCell(value) {
  return {
    value,
    observers: [],
    setValue(value) {
      this.value = value;
      // after setting the value, update observers
      this.observers.forEach(observer => observer.update());
    },
    subscribe(observer) {
      this.observers.push(observer);
    }
  };
}

export function ComputeCell(cells, compute) {
  this.callbacks = [];
  // compute the initial value
  this.value = compute(cells);
  // subscribe self to dependencies
  cells.forEach(cell => cell.subscribe(this));

  // forward listeners to dependencies
  this.subscribe = listener => cells.forEach(cell => cell.subscribe(listener));

  this.addCallback = callback => {
    // if the callback had a null index
    if (callback.index === null) {
      // make it's index equal to the length of the callbacks array
      callback.setIndex(this.callbacks.length);
      // push it to the array
      this.callbacks.push(callback);
    }
  };

  this.removeCallback = callback => {
    // if the callback index is not null
    // then it is still attached to the cell
    if (callback.index !== null) {
      // mutate the callbacks by splicing 1 item at the callback index
      this.callbacks.splice(callback.index, 1);
      // update all callbacks with their new indeces
      this.callbacks.forEach((callback, index) => callback.setIndex(index));
      // finish the removal by setting index to null
      callback.setIndex(null);
    }
  };

  this.update = () => {
    // compute the next value
    const next = compute(cells);
    // if its the same, skip
    if (next !== this.value) {
      this.value = next;
      // after assigning the new value, invoke all callbacks with this object
      this.callbacks.forEach(callback => callback.update(this));
    }
  };

  return this;
}

export function CallbackCell(callback) {
  return {
    values: [],
    update(cell) {
      // push every update
      this.values.push(callback(cell));
    },
    index: null,
    setIndex(index) {
      // index setter
      this.index = index;
    }
  };
}
