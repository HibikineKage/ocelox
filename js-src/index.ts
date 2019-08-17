type StateValue<S> = null | boolean | string | State<S> | number;
interface State<S> {
  update(callback: (state: S) => S);
  delete(key: keyof S);
  [K in keyof S]: S[K];
}
class Store<S> {
  state: State<S> = new Proxy({} as S, {});
  set<K extends keyof S>(key: K, value: S[K]) {
  }
}
