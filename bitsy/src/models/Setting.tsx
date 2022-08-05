import Account from './Account';
import SettingsMetadata from './../settings.json';

const _SettingsMetadata = SettingsMetadata as any;

interface SettingMetadata {
  name: string;
  description: string;
  account_type: string;
}

export default class Setting {
  key: string;
  value: boolean;
  account: Account;
  constructor(key: string, value: boolean, account: Account) {
    this.key = key;
    this.value = value;
    this.account = account;
  }

  description(): string {
    return _SettingsMetadata[this.key].description;
  }
}
