export interface NavigationProps {
  navigation: any;
}
export type Option<T> = T | null | undefined;

export enum ActionState {
  success = 'success',
  pending = 'pending',
  error = 'error',
  none = 'none',
}

export type ErrorMessage = string;
