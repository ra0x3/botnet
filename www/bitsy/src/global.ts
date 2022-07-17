import ethers from 'ethers';



declare global {
    interface Window {
        ethereum?: ethers.providers.ExternalProvider;
      }   
      
      export type Option<T> = T | null | undefined;
      
      export interface HistoryProps {
          history: any;
          match: any;
      }
      
      interface IntrinsicElements {
        string: any
    }
  }