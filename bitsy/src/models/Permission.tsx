import Account from './Account';
import Document from './Document';
import ThirdParty from './ThirdParty';

export default class Permission {
  uuid: string;
  key: string;
  document: Document;
  value: number;
  account: Account;
  party: ThirdParty;
  ttl: number;
  created_at: number;

  constructor(
    uuid: string,
    key: string,
    document: Document,
    value: number,
    account: Account,
    party: ThirdParty,
    ttl: number,
    created_at: number,
  ) {
    this.uuid = uuid;
    this.key = key;
    this.document = document;
    this.value = value;
    this.account = account;
    this.party = party;
    this.ttl = ttl;
    this.created_at = created_at;
  }

  static fromObject(object: {[key: string]: any}): Permission {
    const {uuid, key, document, value, account, party, ttl, created_at} = object;
    const doc = Document.fromObject(document);
    const acct = Account.fromObject(account);
    const third_party = ThirdParty.fromObject(party);

    return new Permission(uuid, key, doc, value, acct, third_party, ttl, created_at);
  }

  static fromJSON(json: string): Permission {
    return Permission.fromObject(JSON.parse(json));
  }
}
