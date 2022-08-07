import React from 'react';
import Web3 from '../services/Web3';
import {Flex, Box, Text, Button} from 'rebass';
import {Input, Label, Textarea} from '@rebass/forms';
import {httpRequest, routeTo, ornull} from '../utils';
import Session from '../services/Session';
import {TEST_MNEMONIC} from '../const';
import Account from '../models/Account';

interface LandingViewProps extends HistoryProps {}

interface LandingViewState {
  showSignupModal: boolean;
  showLoginModal: boolean;
  currentPartyName: string;
  currentPartyAddress: string;
  mnemonic: {
    [key: number]: string;
  };
}

class LandingView extends React.Component<LandingViewProps, LandingViewState> {
  constructor(props: LandingViewProps) {
    super(props);
    this.state = {
      showSignupModal: false,
      showLoginModal: false,
      currentPartyName: '',
      currentPartyAddress: '',
      mnemonic: (() => {
        let result: {[key: number]: string} = {};
        for (let i = 1; i < 25; i++) {
          result[i] = '';
        }
        return result;
      })(),
    };

    this.handleCloseSignupModal = this.handleCloseSignupModal.bind(this);
    this.updateCredentialState = this.updateCredentialState.bind(this);
  }

  componentDidMount() {
    if (Session.hasSession()) {
      return routeTo('/dashboard');
    }
  }

  handleOpenSignupModal() {
    this.setState({showSignupModal: true});
  }

  handleCloseSignupModal() {
    this.setState({showSignupModal: false});
  }

  updateCredentialState(key: string, value: string) {
    this.setState({[key]: value} as any);
  }

  async submitCredentials() {
    const mnemonic = Object.values(this.state.mnemonic).join(' ');
    const account = Web3.loadWalletFromMnemonic(TEST_MNEMONIC);
    if (!account) {
      alert('Could not load wallet.');
      return;
    }

    const {data, error} = await httpRequest({
      url: '/account/third-party',
      method: 'POST',
      data: {
        name: ornull(this.state.currentPartyName, ''),
        // pubkey: ornull(account.publicKey, ''),
        address: ornull(this.state.currentPartyAddress, ''),
      },
    });

    Session.save(Account.fromObject(data.account));

    routeTo('/dashboard');
  }

  renderSignupModal() {
    if (this.state.showSignupModal) {
      return (
        <Flex
          sx={{
            border: '1px solid blue',
            width: 600,
            height: 600,
            zIndex: 10,
            backgroundColor: '#FFF',
            position: 'absolute',
          }}
        >
          <Flex
            sx={{
              border: '1px solid red',
            }}
            flexDirection={'column'}
          >
            <Button
              sx={{cursor: 'pointer'}}
              variant="primary"
              onClick={this.handleCloseSignupModal}
            >
              Close Modal
            </Button>
            <Flex
              flexDirection={'column'}
              justifyContent={'space-between'}
              sx={{border: '1px solid blue', width: 600, padding: 50}}
            >
              <Flex sx={{border: '1px solid red'}} flexDirection={'column'}>
                <Label>Name</Label>
                <Input
                  value={this.state.currentPartyName}
                  onChange={(e) => this.setState({currentPartyName: e.target.value})}
                />
              </Flex>
              <Flex sx={{border: '1px solid red'}} flexDirection={'column'}>
                <Label>Address</Label>
                <Input
                  value={this.state.currentPartyAddress}
                  onChange={(e) => this.setState({currentPartyAddress: e.target.value})}
                />
              </Flex>
              <Flex
                sx={{
                  marginBottom: 20,
                  height: 300,
                  flexDirection: 'column',
                  alignItems: 'center',
                  border: '1px solid green',
                }}
                justifyContent={'space-between'}
              >
                <Label htmlFor="comment">Mnemnonic Phrase</Label>
                <Textarea
                  height={200}
                  onChange={(e) => this.setState({mnemonic: e.target.value})}
                />
                <Flex sx={{border: '1px solid red', width: 300}} justifyContent={'space-between'}>
                  <Button
                    sx={{cursor: 'pointer'}}
                    variant="primary"
                    onClick={async () => await this.submitCredentials()}
                  >
                    Submit
                  </Button>

                  <Button
                    sx={{cursor: 'pointer'}}
                    variant="primary"
                    onClick={async () => await this.submitCredentials()}
                  >
                    Close
                  </Button>
                </Flex>
              </Flex>
            </Flex>
          </Flex>
        </Flex>
      );
    }
  }

  render() {
    return (
      <Flex
        flexDirection={'row'}
        justifyContent={'center'}
        alignItems={'center'}
        sx={{width: '100%', border: '1px solid blue'}}
      >
        <Flex flexDirection={'column'} sx={{border: '1px solid red', width: 600, height: 600}}>
          {this.renderSignupModal()}
          <Button
            variant={'primary'}
            sx={{marginBottom: 20, cursor: 'pointer'}}
            onClick={() => this.handleOpenSignupModal()}
          >
            Signup
          </Button>

          <Button
            variant={'primary'}
            sx={{marginBottom: 20, cursor: 'pointer'}}
            onClick={async () => await Web3.metaMaskLogin()}
          >
            Login
          </Button>
        </Flex>
      </Flex>
    );
  }
}

export default LandingView;
