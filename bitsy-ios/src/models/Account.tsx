import {Option} from '../global';

export default class Account {
  pubkey: Option<string>;
  address: string;
  password_hash: string;
  created_at: number;
  jwt: string;
  nonce: Option<string>;
  constructor(
    pubkey: string,
    address: string,
    password_hash: string,
    created_at: number,
    jwt: string,
    nonce: Option<string>,
  ) {
    this.pubkey = pubkey;
    this.address = address;
    this.password_hash = password_hash;
    this.created_at = created_at;
    this.jwt = jwt;
    this.nonce = nonce;
  }

  static fromObject(object: {[key: string]: any}): Account {
    const {pubkey, address, password_hash, created_at, jwt, nonce} = object;
    return new Account(pubkey, address, password_hash, created_at, jwt, nonce);
  }

  toJSON(): string {
    return JSON.stringify({
      pubkey: this.pubkey,
      address: this.address,
      password_hash: this.password_hash,
      created_at: this.created_at,
      jwt: this.jwt,
      nonce: this.nonce,
    });
  }

  static fromJSON(json: string): Account {
    return Account.fromObject(JSON.parse(json));
  }
}
