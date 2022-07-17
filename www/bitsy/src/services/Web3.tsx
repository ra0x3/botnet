import W3 from 'web3';
import crypto from 'crypto';
import {binToHex, hexToBin, instantiateSecp256k1} from '@bitauth/libauth';
import Account from '../models/Account';
import Session from './Session';
import {signaturePrompt, INFURA_PROVIDER_URL, TEST_MNEMONIC} from '../const';
import {httpRequest, routeTo} from '../utils';
import {Wallet, utils} from 'ethers';

export type EthAddress = string;

interface EthAccount {
  wallet: any;
  hdNode: any;
}

class Web3_ {
  providerHost: string;
  provider: any;

  constructor() {
    this.providerHost = Web3_.getProviderHost();
    this.provider = new W3(this.providerHost);
  }

  private static getProviderHost() {
    if (window && window.ethereum && window.ethereum.host) {
      return window.ethereum.host;
    }
    return INFURA_PROVIDER_URL;
  }

  async uncompressPubkey(compressed: string): Promise<number[]> {
    // https://bitcoin.stackexchange.com/a/93920
    const secp256k1 = await instantiateSecp256k1();
    const compressedBin = hexToBin(compressed.slice(2));
    const uncompressed = secp256k1.uncompressPublicKey(compressedBin);
    const hex = '0x' + binToHex(uncompressed);

    return W3.utils.hexToBytes(hex);
  }

  static generateNonce(): string {
    return crypto.randomBytes(32).toString('hex');
  }

  static async verifyNonceAndSignature(
    nonce: string,
    account: EthAddress,
    signature: string,
    input: string,
  ): Promise<Option<Account>> {
    const {data, error} = await httpRequest({
      url: `/account/auth/verify`,
      method: 'POST',
      data: {
        nonce,
        address: account,
        signature,
        input,
      },
    });

    if (error) {
      console.error(`Could not verify nonce signature on backend: ${error}.`);
      return null;
    }

    if (data && data.success && !data.success) {
      console.error(`Authenication verification failed.`, data);
      return null;
    }

    return Account.fromObject(data);
  }

  static async signMessageWithNonce(
    nonce: string,
    account: EthAddress,
  ): Promise<Option<{input: string; signature: string}>> {
    const input = signaturePrompt(nonce);
    try {
      const signature = await window.ethereum!.request!({
        method: 'personal_sign',
        params: [input, account, ''],
      });
      return {signature, input};
    } catch (e: any) {
      console.error(`Could not sign the request:`, e);
      return null;
    }
  }

  async metaMaskLogin(): Promise<Option<Account>> {
    if (!window.ethereum) {
      alert('Ethereum not enabled.');
    } else {
      try {
        const [account] = await window.ethereum!.request!({
          method: 'eth_requestAccounts',
        });

        const nonce = Web3_.generateNonce();

        const result = await Web3_.signMessageWithNonce(nonce, account);

        if (!result) {
          console.error(`Could not get a nonce`);
          return null;
        }

        const user = (await Web3_.verifyNonceAndSignature(
          nonce,
          account,
          result.signature,
          result.input,
        )) as Account;

        Session.save(user);

        routeTo('/dashboard');
      } catch (e: any) {
        console.error(`Could not get Metamask wallet:`, e);
        return null;
      }
    }
  }

  loadWalletFromMnemonic(mnemonic: string): Option<utils.HDNode> {
    // https://ethereum.stackexchange.com/a/84297
    try {
      const node = utils.HDNode.fromMnemonic(mnemonic);
      // https://docs.ethers.io/ethers.js/v3.0/html/api-advanced.html?highlight=hdnode
      return node.derivePath("m/44'/60'/0'/0/0");
    } catch (e: any) {
      console.error('Could not load wallet: ', e);
      return null;
    }
  }
}

const Web3 = new Web3_();

export default Web3;
