import DocumentBlob from './DocumentBlob';
import Account from './Account';

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
