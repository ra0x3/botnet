import ThirdParty from './ThirdParty';
import AccessToken from './AccessToken';

export enum WebhookType {
  Incoming = 'Incoming',
  Outgoing = 'Outgoing',
}

export default class Webhook {
  uuid: string;
  third_party: ThirdParty;
  type: WebhookType;
  name: string;
  endpoint: string;
  active: boolean;
  constructor(
    uuid: string,
    third_party: ThirdParty,
    type: WebhookType,
    name: string,
    endpoint: string,
    active: boolean,
  ) {
    this.uuid = uuid;
    this.third_party = third_party;
    this.type = type;
    this.name = name;
    this.endpoint = endpoint;
    this.active = active;
  }

  static fromObject(object: {[key: string]: any}): Webhook {
    const {uuid, third_party, type, name, endpoint, active} = object;
    return new Webhook(uuid, third_party, type, name, endpoint, active);
  }
}
