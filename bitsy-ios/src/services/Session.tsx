import {AsyncStorage} from 'react-native';
import Account from '../models/Account';
import {Option} from '../global';

export default class Session {
  static async hasSession() {
    const session = Session.getSession();
    return !!session;
  }

  static async getSession(): Promise<Account> {
    const result = (await AsyncStorage.getItem('x-bitsy')) as string;
    console.log('>>> RESULT ', result);
    return Account.fromJSON(result);
  }

  static save(account: Account, callback: any) {
    AsyncStorage.setItem('x-bitsy', account.toJSON());
    return callback();
  }
}
