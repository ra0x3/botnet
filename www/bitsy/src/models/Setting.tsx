import Account from './Account';

export default class Setting {
  key: string;
  value: boolean;
  account: Account;
  constructor(key: string, value: boolean, account: Account) {
    this.key = key;
    this.value = value;
    this.account = account;
  }
}
