import Page from './Page';
import { Store } from 'redux';

namespace Resolver {
  export enum Type {
    Uri,
    UriGenerator,
    Generator  
  }

  export interface Uri {
    type: Type.Uri,
    uri: string,
    generator: (content: string, page: Page.Unresolved) => Promise<Page.Resolved[]>;
  }
}

type Resolver = Resolver.Uri;

export default Resolver;