import Page from '../state/Page';
import Resolver from '../state/Resolver';
import store from '../store';
import Dict from '../Dict';

export interface Set {
  type: 'page-set',
  pages: Page[]
}

export interface Resolve {
  type: 'page-resolve',
  id: string
}

export interface Remove {
  type: 'page-remove',
  id: string
}

export type Action = Set | Remove | Resolve;

export interface PathPart {
  id: string,
  children: Dict<PathPart>
}

export interface State {
  root: string,
  pages: Dict<Page>,
  byPath: Dict<PathPart>,
  byParent: Dict<string>
}

const MANIFEST = (window as any).MANIFEST;

export namespace State {
  const pages = Object.keys(MANIFEST.pages).map(key => ({ [MANIFEST.pages[key].id]: {
    state: Page.State.Unresolved,
    ...MANIFEST.pages[key]
  }})).reduce((l, r) => ({ ...l, ...r }), {});
  
  const byPath = ((pages: Dict<Page>) => {
    const children = new Set<string>();
    console.log(pages);
    Dict.forEach(pages, page => {
      if (!page || !page.children) return;
      for (let i = 0; i < page.children.length; ++i) {
        const child = page.children[i];
        children.add(child);
      }
    });

    const roots = Dict.filter(pages, page => !children.has(page.id));


  })(pages);

  const byParents = ((pages: Dict<Page>) => {
    const visited = new Set<string>();
    const children = new Set<string>();
    console.log(pages);
    Dict.forEach(pages, page => {
      if (!page || !page.children) return;
      for (let i = 0; i < page.children.length; ++i) {
        const child = page.children[i];
        children.add(child);
      }
    });

    const ret = {};
    const roots = Dict.keys(Dict.filter(pages, page => !children.has(page.id)));

    let q = [ ...roots.map(root => ([null, root])) ] as [string | null, string][];
    while (q.length > 0) {
      const [parent, current] = q.shift() as [string | null, string];
      if (parent !== null) ret[current] = parent;
      
      if (!pages[current]) {
        console.log(`WARNING: Page ${current} is undefined. This will cause issues.`);
        continue;
      }
      
      if (visited.has(current)) {
        console.log('Cycle detected!', current);
        continue;
      }

      visited.add(current);

      q.push(...(pages[current].children.map(child => ([ current, child ])) as [string, string][]));
    }

    return ret;
  })(pages);
  
  export const DEFAULT: State = {
    root: MANIFEST.root,
    pages,
    byPath: {},
    byParent: byParents
  }
}

const readAll = async (stream: ReadableStreamDefaultReader<Uint8Array>) => {
  const chunks: Uint8Array[] = [];
  let size = 0;
  for (;;) {
    const res = await stream.read();
    
    if (res.value) {
      chunks.push(res.value);
      size += res.value.byteLength;
    }
    
    if (res.done) break;
  }

  const buffer = new ArrayBuffer(size);
  const view = new Uint8Array(buffer);

  let total = 0;
  for (let i = 0; i < chunks.length; ++i) {
    const chunk = chunks[i];
    view.set(chunk, total);
    total += chunk.byteLength;
  }

  return buffer;
};

export const resolve = async (page: Page.Unresolved) => {
  if (!page.resolvable) {
    console.log(page, 'not resolvable');
    window.setTimeout(() => {
      store.dispatch({
        type: 'page-set',
        pages: [{
          state: Page.State.Resolved,
          id: page.id,
          name: page.name,
          content_type: page.content_type,
          children: page.children,
          metadata: page.metadata,
          renderer: page.renderer,
          url: page.url,
          hidden: page.hidden,
          resolvable: page.resolvable,
          content: undefined
        } as Page.Resolved]
      });
    }, 0);
    return;
  }
  const response = await fetch(page.url);

  const body = response.body;
  if (!body) {
    console.log(`ERROR: Expected GET ${page.id}.data to return a body`);
    return;
  }

  const buffer = await readAll(body.getReader());
  const content = new TextDecoder("utf-8").decode(buffer);
  
  store.dispatch({
    type: 'page-set',
    pages: [{
      state: Page.State.Resolved,
      id: page.id,
      name: page.name,
      content_type: page.content_type,
      children: page.children,
      metadata: page.metadata,
      renderer: page.renderer,
      hidden: page.hidden,
      url: page.url,
      resolvable: page.resolvable,
      content
    } as Page.Resolved]
  });
};

export default (state: State = State.DEFAULT, action: Action): State => {
  switch (action.type) {
    case 'page-set': {
      const copy = {
        ...state,
        pages: {
          ...state.pages
        }
      };

      for (let i = 0; i < action.pages.length; ++i) {
        const page = action.pages[i];
        copy.pages[page.id] = page;
      }

      return copy;
    }
    case 'page-resolve': {
      const page = state.pages[action.id];
      
      if (!page) {
        console.log(`ERROR: Page ${action.id} can't be resolved. It doesn't exist`);
        return state;
      }

      // If the state is already resolved or resolving, there's nothing to do
      if (page.state != Page.State.Unresolved) return state;

      resolve(page);
      
      return {
        ...state,
        pages: {
          ...state.pages,
          [page.id]: {
            state: Page.State.Resolving,
            id: page.id,
            metadata: page.metadata,
            content_type: page.content_type,
            name: page.name,
            renderer: page.renderer,
            children: page.children,
            resolvable: page.resolvable,
            hidden: page.hidden,
            url: page.url
          }
        }
      };
    }
    case 'page-remove': {
      return state;
    }
  }

  return state;
};