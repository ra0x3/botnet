import Account from './Account';
import ThirdParty from './ThirdParty';

export default class ThirdPartyAccount {
  account: Account;
  third_party: ThirdParty;
  constructor(account: Account, third_party: ThirdParty) {
    this.account = account;
    this.third_party = third_party;
  }

  static fromObject(object: {[key: string]: any}): ThirdPartyAccount {
    const {account, third_party} = object;
    const account_ = Account.fromObject(account);
    const party_ = ThirdParty.fromObject(third_party);
    return new ThirdPartyAccount(account_, party_);
  }

  toJSON(): string {
    return JSON.stringify({
      account: this.account.toJSON(),
      third_party: this.third_party.toJSON(),
    });
  }

  static fromJSON(json: string): ThirdPartyAccount {
    return ThirdPartyAccount.fromObject(JSON.parse(json));
  }
}
