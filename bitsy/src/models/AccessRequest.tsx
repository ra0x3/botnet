import Account from './Account';
import Document from './Document';
import ThirdParty from './ThirdParty';

export enum AccessRequestStatus {
  Granted = 'Granted',
  Denied = 'Denied',
  Pending = 'Pending',
}

export default class AccessRequest {
  uuid: string;
  third_party: ThirdParty;
  account: Account;
  status: AccessRequestStatus;
  document: Document;
  callback_url: string;
  callback_data: {[key: string]: any};
  created_at: number;
  expiry: number;

  constructor(
    uuid: string,
    third_party: ThirdParty,
    account: Account,
    status: AccessRequestStatus,
    document: Document,
    callback_url: string,
    callback_data: {[key: string]: any},
    created_at: number,
    expiry: number,
  ) {
    this.uuid = uuid;
    this.third_party = third_party;
    this.account = account;
    this.status = status;
    this.document = document;
    this.callback_url = callback_url;
    this.callback_data = callback_data;
    this.created_at = created_at;
    this.expiry = expiry;
  }

  formattedDescription() {
    return `${this.third_party.name} requested access to ${
      this.document.name
    } on ${this.getPrettyTime(this.created_at)}.`;
  }

  getPrettyTime(t: number) {
    // TODO: Fix
    return t.toString();
  }

  formattedTitle() {
    return `${this.third_party.name} wants to access ${this.document.name}`;
  }
}
