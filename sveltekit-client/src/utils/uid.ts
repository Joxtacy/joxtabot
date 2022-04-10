function* uidGenerator(i = 1) {
  while (true) {
    yield i++;
  }
}

const uidGen = uidGenerator();
export default uidGen;
