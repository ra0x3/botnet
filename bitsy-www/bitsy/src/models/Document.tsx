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

  constructor(cid: string, name: string, blob: DocumentBlob, account: Account, key_img: string) {
    this.cid = cid;
    this.name = name;
    this.blob = blob;
    this.account = account;
    this.key_img = key_img;
  }
}
