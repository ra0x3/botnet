import React from 'react';
import Web3 from '../services/Web3';
import {Flex, Box, Text, Button} from 'rebass';
import {Input, Label} from '@rebass/forms';
import {httpRequest, routeTo} from '../utils';
import Session from '../services/Session';
import {TEST_MNEMONIC} from '../const';
import Account from '../models/Account';

interface LandingViewProps extends HistoryProps {}

interface LandingViewState {
  showSignupModal: boolean;
  showLoginModal: boolean;
  currentPartyName: string;
  mnemonic: {
    [key: number]: string;
  };
}

interface MnemonicWordProps {
  position: number;
  updateMnemonicState: any;
  value: string;
}

const MnemonicWord = ({value, position, updateMnemonicState}: MnemonicWordProps) => {
  return (
    <Box sx={{border: '1px solid red', width: 100, borderRadius: 10}}>
      <Label>{position}</Label>
      <Input
        value={value}
        onChange={(e) => {
          updateMnemonicState(position, e.target.value);
        }}
      />
    </Box>
  );
};

class LandingView extends React.Component<LandingViewProps, LandingViewState> {
  constructor(props: LandingViewProps) {
    super(props);
    this.state = {
      showSignupModal: false,
      showLoginModal: false,
      currentPartyName: '',
      mnemonic: (() => {
        let result: {[key: number]: string} = {};
        for (let i = 1; i < 25; i++) {
          result[i] = '';
        }
        return result;
      })(),
    };

    this.handleCloseSignupModal = this.handleCloseSignupModal.bind(this);
    this.updateMnemonicState = this.updateMnemonicState.bind(this);
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

  updateMnemonicState(position: number, word: string) {
    this.setState({mnemonic: {...this.state.mnemonic, [position]: word}});
  }

  async submitmnemonic() {
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
        name: this.state.currentPartyName === '' ? null : this.state.currentPartyName,
        pubkey: account!.publicKey,
        address: account!.address,
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
            <Flex sx={{width: '100%'}} alignItems={'center'} justifyContent={'center'}>
              <Flex sx={{border: '1px solid red'}} flexDirection={'column'}>
                <Label>Name</Label>
                <Input
                  value={this.state.currentPartyName}
                  onChange={(e) => this.setState({currentPartyName: e.target.value})}
                />
              </Flex>
            </Flex>
            <Flex
              flexDirection={'column'}
              justifyContent={'space-between'}
              sx={{border: '1px solid blue', width: 600, padding: 50}}
            >
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[1]}
                  position={1}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[2]}
                  position={2}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[3]}
                  position={3}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[4]}
                  position={4}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[5]}
                  position={5}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[6]}
                  position={6}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[7]}
                  position={7}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[8]}
                  position={8}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[9]}
                  position={9}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[10]}
                  position={10}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[11]}
                  position={11}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[12]}
                  position={12}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[13]}
                  position={13}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[14]}
                  position={14}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[15]}
                  position={15}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[16]}
                  position={16}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[17]}
                  position={17}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[18]}
                  position={18}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[19]}
                  position={19}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[20]}
                  position={20}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Flex sx={{marginBottom: 20}} justifyContent={'space-between'}>
                <MnemonicWord
                  value={this.state.mnemonic[21]}
                  position={21}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[22]}
                  position={22}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[23]}
                  position={23}
                  updateMnemonicState={this.updateMnemonicState}
                />
                <MnemonicWord
                  value={this.state.mnemonic[24]}
                  position={24}
                  updateMnemonicState={this.updateMnemonicState}
                />
              </Flex>
              <Button
                sx={{cursor: 'pointer'}}
                variant="primary"
                onClick={async () => await this.submitmnemonic()}
              >
                Submit
              </Button>
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
