import { ConnectedRouter, connectRouter, routerMiddleware } from 'connected-react-router';
import { createStore, combineReducers, compose, applyMiddleware } from 'redux';

import page, { State as PageState } from './store/page';

import { createBrowserHistory } from 'history';

export interface State {
  page: PageState,
  router: any 
}

export const history = createBrowserHistory() as any;

export default createStore(combineReducers({
  page,
  router: connectRouter(history)
}), undefined, compose(
  applyMiddleware(routerMiddleware(history))
))
