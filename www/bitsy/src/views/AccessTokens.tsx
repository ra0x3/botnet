import React from 'react';
import {Flex, Box, Text, Button} from 'rebass';
import {TrashOutline} from 'react-ionicons';
import {Label, Input, Switch, Select} from '@rebass/forms';
import AccessToken from './../models/AccessToken';
import Account from './../models/Account';
import TopNavigation from './../components/TopNavigation';
import {httpRequest} from '../utils';
import Session from '../services/Session';

interface AccessTokenItemProps {
  token: AccessToken;
  removeAccessTokenItem: any;
  updateAccessTokenItem: any;
  account: Account;
}

interface CurrentAccessTokenItemProps {
  name: string;
}

const AccessTokenItem = ({
  token,
  removeAccessTokenItem,
  updateAccessTokenItem,
  account,
}: AccessTokenItemProps) => {
  const toggleAccessTokenActive = async (token: AccessToken, account: Account) => {
    const {data, error} = await httpRequest({
      url: '/access-token',
      method: 'PUT',
      data: {
        uuid: token.uuid,
      },
      headers: {
        Authorization: account.jwt,
      },
    });

    if (error) {
      alert(`Could not update access token.`);
      console.error(`Could not update access token: `, error);
    }

    updateAccessTokenItem(data);
  };

  const deleteAccessToken = async () => {
    const {error} = await httpRequest({
      url: '/access-token',
      method: 'DELETE',
      data: {
        uuid: token.uuid,
      },
      headers: {
        Authorization: account.jwt,
      },
    });

    if (error) {
      alert(`Could not delete token: ${JSON.stringify(error)}`);
      return;
    }

    removeAccessTokenItem(token);
  };

  return (
    <Flex
      sx={{
        border: '1px solid red',
        width: '100%',
        height: 150,
        marginBottom: 10,
        cursor: 'pointer',
      }}
    >
      <Flex sx={{border: '1px solid green', width: '100%'}} flexDirection={'column'}>
        <Flex sx={{border: '1px solid red'}}>{token.uuid}</Flex>
        <Flex sx={{border: '1px solid red'}}>{token.name}</Flex>
        <Flex sx={{border: '1px solid red'}}>{token.expiry}</Flex>
        <Flex sx={{border: '1px solid blue'}} justifyContent={'space-evenly'}>
          <Flex>
            <Box sx={{border: '1px solid orange'}}>
              <TrashOutline
                onClick={async () => await deleteAccessToken()}
                style={{cursor: 'hover'}}
                height={'25px'}
                width={'25px'}
              />
            </Box>
            <Switch
              checked={token.active}
              backgroundColor={'white'}
              onClick={async () => await toggleAccessTokenActive(token, account)}
            />
          </Flex>
        </Flex>
      </Flex>
    </Flex>
  );
};

interface AccessTokensViewProps extends HistoryProps {}

interface AccessTokensViewState {
  account: Account;
  items: Array<AccessToken>;
  showAddAccessTokenModal: boolean;
  currentAccessToken: CurrentAccessTokenItemProps;
}

class AccessTokensView extends React.Component<AccessTokensViewProps, AccessTokensViewState> {
  constructor(props: AccessTokensViewProps) {
    super(props);
    this.state = {
      account: Session.getSession(),
      showAddAccessTokenModal: false,
      currentAccessToken: {
        name: '',
      },
      items: [],
    };

    this.removeAccessTokenItem = this.removeAccessTokenItem.bind(this);
    this.updateAccessTokenItem = this.updateAccessTokenItem.bind(this);
  }

  async componentDidMount() {
    const {data, error} = await httpRequest({
      url: '/access-token',
      method: 'GET',
      headers: {
        Authorization: this.state.account.jwt,
      },
    });

    if (error) {
      alert(`Failed to fetch tokens`);
      this.setState({items: []});
      return;
    }

    this.setState({items: data});
  }

  updateAccessTokenItem(token: AccessToken) {
    const findWebhook = (t: AccessToken) => t.uuid == token.uuid;
    const index = this.state.items.findIndex(findWebhook);
    const items = this.state.items;
    items[index] = token;
    this.setState({items});
  }

  removeAccessTokenItem(token: AccessToken) {
    const updated = this.state.items.filter((item) => item.uuid !== token.uuid);
    this.setState({items: updated});
  }

  updateCurrentAccessToken(key: string, value: string | boolean) {
    this.setState({currentAccessToken: {...this.state.currentAccessToken, [key]: value}});
  }

  async saveCurrentAccessToken() {
    const {data, error} = await httpRequest({
      url: '/access-token',
      method: 'POST',
      data: {
        ...this.state.currentAccessToken,
      },
      headers: {
        Authorization: this.state.account.jwt,
      },
    });

    if (error) {
      console.log(`Failed to add token: `, error);
      alert(error);
      return null;
    }

    const token = AccessToken.fromObject(data);
    this.setState({items: [...this.state.items, token]}, () => {
      this.setState({showAddAccessTokenModal: false});
    });

    throw Error;
  }

  renderAddAccessTokenModal() {
    if (this.state.showAddAccessTokenModal) {
      return (
        <Flex
          sx={{
            border: ' 1px solid black',
            width: 500,
            position: 'absolute',
            zIndex: 10,
            backgroundColor: '#fff',
          }}
        >
          <Flex sx={{border: '1px solid red', width: 500}} flexDirection={'column'}>
            <Box sx={{border: '1px solid blue', marginBottom: 20, padding: 10}}>
              <Label>Name</Label>
              <Input
                type="text"
                value={this.state.currentAccessToken.name}
                onChange={(e) => this.updateCurrentAccessToken('name', e.target.value)}
              />
              <Text sx={{fontSize: 12, color: 'grey', marginTop: 1}}>Some details</Text>
            </Box>
            <Flex>
              <Button
                sx={{
                  width: 100,
                  cursor: 'pointer',
                  position: 'relative',
                  right: 0,
                  marginRight: 30,
                }}
                onClick={() => this.setState({showAddAccessTokenModal: false})}
              >
                Cancel
              </Button>
              <Button
                sx={{width: 100, cursor: 'pointer', position: 'relative', right: 0}}
                onClick={async () => this.saveCurrentAccessToken()}
              >
                Save
              </Button>
            </Flex>
          </Flex>
        </Flex>
      );
    }
  }

  renderFlatList() {
    return this.state.items.map((item: AccessToken, i: number) => {
      return (
        <AccessTokenItem
          key={String(i)}
          token={item}
          updateAccessTokenItem={this.updateAccessTokenItem}
          removeAccessTokenItem={this.removeAccessTokenItem}
          account={this.state.account}
        />
      );
    });
  }

  render() {
    return (
      <Flex justifyContent={'center'} alignItems={'center'} flexDirection={'column'}>
        <TopNavigation />
        <Flex
          alignItems={'center'}
          flexDirection={'column'}
          sx={{border: '1px solid black', width: 600, height: 800, padding: 10}}
        >
          <Button
            sx={{width: 100, cursor: 'pointer', position: 'relative', right: 0}}
            onClick={() => this.setState({showAddAccessTokenModal: true})}
          >
            Add
          </Button>
          {this.renderAddAccessTokenModal()}
          {this.renderFlatList()}
        </Flex>
      </Flex>
    );
  }
}

export default AccessTokensView;
