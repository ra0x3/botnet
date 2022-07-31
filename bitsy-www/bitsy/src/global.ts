import ethers from 'ethers';

declare global {
    export interface Window {
      ethereum?: ethers.providers.ExternalProvider;
    }   
      
    export type Option<T> = T | null | undefined;
      
    export interface HistoryProps {
        history: any;
        match: any;
    }
      
    export interface IntrinsicElements {
      string: any
    }

    export enum ActionState {
      success ='success',
      pending = 'pending',
      error = 'error',
      none = 'none',
    }
}


export enum ActionState {
  success ='success',
  pending = 'pending',
  error = 'error',
  none = 'none',
}
