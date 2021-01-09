type Dict<T> = { [id: string]: T };

namespace Dict {
  export const keys = <T>(dict: Dict<T>): string[] => Object.keys(dict);

  export const filter = <T>(dict: Dict<T>, pred: (value: T, id: string) => boolean): Dict<T> => {
    const ret = {};
    
    const ids = keys(dict);
    for (let i = 0; i < ids.length; ++i) {
      const id = ids[i];
      const value = dict[id];
      if (!pred(value, id)) continue;
      ret[id] = value;
    }

    return ret;
  }

  export const subset = <T, U>(dict: Dict<T>, keys: string[]): Dict<U> => {
    const ret = {};
    
    for (let i = 0; i < keys.length; ++i) {
      const id = keys[i];
      const value = dict[id];
      if (!!value) ret[id] = value;
    }

    return ret;
  }

  export const map = <T, U>(dict: Dict<T>, f: (value: T, id: string) => U): Dict<U> => {
    const ret = {};
    
    const ids = keys(dict);
    for (let i = 0; i < ids.length; ++i) {
      const id = ids[i];
      const value = dict[id];
      ret[id] = f(value, id);
    }

    return ret;
  }

  export const forEach = <T>(dict: Dict<T>, f: (value: T, id: string) => void) => {
    const ids = keys(dict);
    for (let i = 0; i < ids.length; ++i) {
      const id = ids[i];
      const value = dict[id];
      f(value, id);
    }
  }

  export const values = <T>(dict: Dict<T>): T[] => {
    const ret: T[] = [];
    
    const ids = keys(dict);
    for (let i = 0; i < ids.length; ++i) {
      const id = ids[i];
      const value = dict[id];
      ret.push(value);
    }

    return ret;
  }

  export const toList = <T>(dict: Dict<T>): [string, T][] => {
    const ret: [string, T][] = [];
    
    const ids = keys(dict);
    for (let i = 0; i < ids.length; ++i) {
      const id = ids[i];
      const value = dict[id];
      ret.push([ id, value ]);
    }

    return ret;
  }
}

export default Dict;