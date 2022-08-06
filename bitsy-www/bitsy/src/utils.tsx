import {AxiosRequestConfig} from 'axios';
import {api} from './services/Axios';

export const routeTo = (path: string) => {
  window.location.href = path;
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
