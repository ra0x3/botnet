export default class ThirdParty {
  uuid: string;
  name: string;
  constructor(uuid: string, name: string) {
    this.uuid = uuid;
    this.name = name;
  }

  static fromObject(object: {[key: string]: any}): ThirdParty {
    const {uuid, name} = object;
    return new ThirdParty(uuid, name);
  }

  toJSON(): string {
    return JSON.stringify({
      uuid: this.uuid,
      name: this.name,
    });
  }
}
