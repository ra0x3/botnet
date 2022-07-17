import React from 'react';
import Web3 from '../services/Web3';
import {Button, Flex, Heading} from 'rebass';
import Account from '../models/Account';
import Session from '../services/Session';
import {routeTo} from '../utils';

interface NavigationProps {}

interface TopNavigationState {
  account: Option<Account>;
}

class TopNavigation extends React.Component<NavigationProps, TopNavigationState> {
  constructor(props: NavigationProps) {
    super(props);
    this.state = {
      account: Session.getSession(),
    };
  }

  _renderRightComponent() {
    if (!this.state.account) {
      return (
        <Button
          variant="primary"
          sx={{
            borderRadius: 10,
            backgroundColor: '#000',
            fontFamily: 'Syne',
            fontWeight: 'bold',
            cursor: 'pointer',
          }}
          onClick={async () => await Web3.metaMaskLogin()}
        >
          Connect
        </Button>
      );
    } else {
      return (
        <Button
          variant="primary"
          sx={{
            borderRadius: 10,
            backgroundColor: '#000',
            fontFamily: 'Syne',
            fontWeight: 'bold',
            cursor: 'pointer',
          }}
          onClick={() => routeTo('/logout')}
        >
          Disconnect
        </Button>
      );
    }
  }

  render() {
    return (
      <Flex sx={{width: '100%'}}>
        <Flex
          justifyContent={'space-between'}
          sx={{
            border: '1px solid black',
            padding: 10,
            width: '100%',
          }}
        >
          <Heading
            sx={{fontFamily: 'Syne', fontWeight: '900', cursor: 'pointer'}}
            onClick={() => (window.location.href = '/dashboard')}
          >
            bitsy
          </Heading>
          <Flex>{this._renderRightComponent()}</Flex>
        </Flex>
      </Flex>
    );
  }
}

export default TopNavigation;
