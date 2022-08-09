import {AxiosRequestConfig} from 'axios';
import {api} from './services/Axios';
import {Option} from './global';

export const generateFakeItems = (item: any, count: number): Array<any> => {
  let items = [];

  for (let i = 0; i < count; i++) {
    items.push(item);
  }

  return items;
};

export const booleanify = (x: number): boolean => {
  return x === 0 ? false : true;
};

export const httpRequest = async (options: AxiosRequestConfig) => {
  try {
    const {data} = await api(options);

    return {data};
  } catch (e: any) {
    console.warn(`HttpRequestError: ${e.toString()}`);
    return {
      error: e.toString(),
    };
  }
};

export const now = (): number => {
  return Date.now() / 1000;
};

export function ornull<T>(x: T, test: Option<T>): Option<T> {
  return x == test ? null : x;
}
