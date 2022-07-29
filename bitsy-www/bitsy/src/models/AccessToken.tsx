export default class AccessToken {
  uuid: string;
  name: string;
  expiry: number;
  active: boolean;
  constructor(uuid: string, name: string, expiry: number, active: boolean) {
    this.uuid = uuid;
    this.name = name;
    this.expiry = expiry;
    this.active = active;
  }

  static fromObject(object: {[key: string]: any}): AccessToken {
    const {uuid, name, active, expiry} = object;
    return new AccessToken(uuid, name, expiry, active);
  }

  toJSON(): string {
    return JSON.stringify({
      uuid: this.uuid,
      name: this.name,
      expiry: this.expiry,
      active: this.active,
    });
  }
}
