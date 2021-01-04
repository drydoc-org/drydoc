import * as React from 'react';
import * as ReactDom from 'react-dom';

import { Provider } from 'react-redux';
import store, { history } from './store';

const mount = window.document.getElementById("mount");

import App from './App';
import { HashRouter } from 'react-router-dom';
import { ConnectedRouter } from 'connected-react-router';

ReactDom.render((
  <HashRouter>
    <Provider store={store}>
      <ConnectedRouter history={history}>
        <App />
      </ConnectedRouter>
    </Provider>
  </HashRouter>
), mount);