import Dict from '../Dict';
import Resolver from './Resolver';

namespace Page {
  export enum State {
    Unresolved,
    Resolving,
    Resolved
  }

  interface Base {
    id: string,
    content_type: string,
    name: string,
    renderer: string,
    metadata: Dict<string>,
    children: string[]
  }
  
  export interface Unresolved extends Base {
    state: State.Unresolved,
    
  }

  export interface Resolving extends Base {
    state: State.Resolving,
  }
  
  export interface Resolved<T = any> extends Base {
    state: State.Resolved,
    content: T,
  }

  export const isResolved = (page: Page) => page.state === State.Resolved;
  export const isResolving = (page: Page) => page.state === State.Resolving;
  export const isUnresolved = (page: Page) => page.state === State.Unresolved;
  export const isTerminal = (page: Page) => page.children.length === 0;
  export const isNonTerminal = (page: Page) => page.children.length !== 0;
}

type Page = Page.Unresolved | Page.Resolving | Page.Resolved;

export default Page;