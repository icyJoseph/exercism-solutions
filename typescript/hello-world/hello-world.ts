class HelloWorld {
  static hello(name?: string): string {
    return `Hello, ${name || "World"}!`;
  }
}

export default HelloWorld;
