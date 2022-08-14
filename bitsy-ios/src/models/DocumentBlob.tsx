export default class DocumentBlob {
  data: any;
  constructor(data: any) {
    this.data = data;
  }

  static fromObject(object: {[key: string]: any}): DocumentBlob {
    const {data} = object;
    return new DocumentBlob(data);
  }

  static fromJSON(json: string): Document {
    return DocumentBlob.fromObject(JSON.parse(json));
  }
}
