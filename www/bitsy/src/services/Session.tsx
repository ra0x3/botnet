import {routeTo, httpRequest} from '../utils';
import Account from '../models/Account';

export default class Session {
  static save(account: Account) {
    return localStorage.setItem('x-bitsy', account.toJSON());
  }

  static getSession(): Account {
    const cache = localStorage.getItem('x-bitsy') as string;
    if (cache === '') {
      throw new Error('No session cached.');
    }
    return Account.fromJSON(cache);
  }

  static hasSession() {
    const cache = localStorage.getItem('x-bitsy');
    // TODO: Need to check that session is valid against backend
    return !!cache;
  }

  static logout() {
    localStorage.removeItem('x-bitsy');
    return routeTo('/');
  }
}
