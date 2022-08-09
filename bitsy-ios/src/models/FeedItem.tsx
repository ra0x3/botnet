import {ethers} from 'ethers';

export enum FeedItemType {
  AccessRequest = 'AccessRequest',
  Document = 'Document',
}

export default class FeedItem {
  title: string;
  subtitle: string;
  text: string;
  type: FeedItemType;
  created_at?: number;

  constructor(
    title: string,
    subtitle: string,
    text: string,
    type: FeedItemType,
    created_at?: number,
  ) {
    this.title = title;
    this.subtitle = subtitle;
    this.text = text;
    this.type = type;
    this.created_at = created_at;
  }

  formattedTime() {
    throw new Error('Not Implemented');
  }

  // TODO: Make this an actual id (HASH)
  id(): string {
    const randi = Math.floor(Math.random() * (10e5 - 1) + 1).toString();

    return this.title + randi;
  }
}
