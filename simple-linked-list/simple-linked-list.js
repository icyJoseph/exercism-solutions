export function Element(element) {
  return {
    value: element,
    next: null,
    prev: null
  };
}

export function List(init = []) {
  const list = init.reduce((acc, curr) => {
    const [next = null] = acc;
    return [{ ...new Element(curr), next }, ...acc];
  }, []);

  return {
    list,
    get length() {
      return this.list.length;
    },
    get head() {
      const [head = null] = this.list;
      return head;
    },
    add(el) {
      const [next] = this.list;
      this.list = [{ ...el, next }, ...this.list];
    },
    reverse() {
      const reversed = this.list.reduceRight((prev, curr) => {
        const [next = null] = prev;
        return [...prev, { ...curr, next }];
      }, []);
      this.list = [...reversed];
      return this;
    },
    toArray() {
      return this.list.map(({ value }) => value);
    }
  };
}
