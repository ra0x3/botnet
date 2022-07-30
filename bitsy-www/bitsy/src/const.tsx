export const signaturePrompt = (s: string): string => {
  return `Welcome to bitsy <3.

  Sign this message to securely log in.

  Nonce: ${s}`;
};

export const INFURA_PROVIDER_URL = `https://mainnet.infura.io/v3/d43cbbb5c7074d3ea28685326166b2e7`;

export const TEST_MNEMONIC =
  'release cargo satoshi penalty security orphan silk input soul region prevent exist';
