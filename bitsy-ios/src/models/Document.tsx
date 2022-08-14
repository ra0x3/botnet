import DocumentBlob from './DocumentBlob';
import Account from './Account';

export enum DocumentType {
  Basic = 'Basic',
  BankCard = 'BankCard',
  BankAccount = 'BankAccount',
}

export default class Document {
  cid: string;
  name: string;
  blob: DocumentBlob;
  account: Account;
  key_img: string;
  created_at: number;

  constructor(
    cid: string,
    name: string,
    blob: DocumentBlob,
    account: Account,
    key_img: string,
    created_at: number,
  ) {
    this.cid = cid;
    this.name = name;
    this.blob = blob;
    this.account = account;
    this.key_img = key_img;
    this.created_at = created_at;
  }

  static fromObject(object: {[key: string]: any}): Document {
    const {cid, name, blob, account, key_img, created_at} = object;
    const docblob = DocumentBlob.fromObject(blob);
    const acct = Account.fromObject(account);
    return new Document(cid, name, docblob, acct, key_img, created_at);
  }

  static fromJSON(json: string): Document {
    return Document.fromObject(JSON.parse(json));
  }
}
