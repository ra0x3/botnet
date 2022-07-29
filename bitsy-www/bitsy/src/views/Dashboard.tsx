import React from 'react';
import {Flex, Box, Text, Button} from 'rebass';
import TopNavigation from './../components/TopNavigation';
import {routeTo} from '../utils';
import Session from '../services/Session';

interface DashboardViewProps extends HistoryProps {}

interface DashboardViewState {
  account: any;
}

class DashboardView extends React.Component<DashboardViewProps, DashboardViewState> {
  constructor(props: DashboardViewProps) {
    super(props);
    this.state = {
      account: Session.getSession(),
    };
  }

  render() {
    return (
      <Flex
        flexDirection={'column'}
        justifyContent={'center'}
        sx={{width: '100%', border: '1px solid blue'}}
      >
        <TopNavigation />
        <Flex
          flexDirection={'column'}
          alignItems={'center'}
          justifyContent={'center'}
          sx={{width: '100%'}}
        >
          <Flex sx={{marginTop: 20}}>
            <Flex
              style={styles.iconBox}
              justifyContent={'center'}
              onClick={() => routeTo('webhooks')}
            >
              Webhooks
            </Flex>
            <Flex
              style={styles.iconBox}
              justifyContent={'center'}
              sx={{marginLeft: 20, marginRight: 20}}
              onClick={() => routeTo('settings')}
            >
              Settings
            </Flex>
            <Flex
              style={styles.iconBox}
              justifyContent={'center'}
              onClick={() => routeTo('access-tokens')}
            >
              Access Tokens
            </Flex>
          </Flex>
        </Flex>
      </Flex>
    );
  }
}

const styles = {
  iconBox: {
    border: '1px solid red',
    width: 250,
    height: 250,
    cursor: 'pointer',
    borderRadius: 15,
  },
};

export default DashboardView;
