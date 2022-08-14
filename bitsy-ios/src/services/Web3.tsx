import W3 from 'web3';
import crypto from 'crypto';
// @ts-ignore
import {binToHex, hexToBin, instantiateSecp256k1} from '@bitauth/libauth';
import Account from '../models/Account';
import {INFURA_PROVIDER_URL, TEST_MNEMONIC} from '../const';
import {Option} from '../global';
import {httpRequest} from '../utils';
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

  loadWalletFromMnemonic(mnemonic: string, password?: string): Option<utils.HDNode> {
    // https://ethereum.stackexchange.com/a/84297
    try {
      const node = utils.HDNode.fromMnemonic(mnemonic, password);
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
