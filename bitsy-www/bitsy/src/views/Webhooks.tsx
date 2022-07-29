import React from 'react';
import {Flex, Box, Text, Button} from 'rebass';
import {TrashOutline} from 'react-ionicons';
import {Label, Input, Switch, Select} from '@rebass/forms';
import Webhook, {WebhookType} from './../models/Webhook';
import TopNavigation from './../components/TopNavigation';
import {httpRequest} from '../utils';
import Account from '../models/Account';
import Session from '../services/Session';

interface WebhookItemProps {
  webhook: Webhook;
  removeWebhookItem: any;
  updateWebhookItem: any;
  account: Account;
}

interface CurrentWebhookItemProps {
  active: boolean;
  type: string;
  name: string;
  endpoint: string;
}

const WebhookItem = ({
  webhook,
  removeWebhookItem,
  updateWebhookItem,
  account,
}: WebhookItemProps) => {
  const toggleWebhookActive = async (webhook: Webhook, account: Account) => {
    const {data, error} = await httpRequest({
      url: '/webhook',
      method: 'PUT',
      data: {
        uuid: webhook.uuid,
      },
      headers: {
        Authorization: account.jwt,
      },
    });

    if (error) {
      alert(`Could not update webhook.`);
      console.error(`Could not update webhook: `, error);
    }

    updateWebhookItem(data);
  };

  const deleteWebhook = async () => {
    const {error} = await httpRequest({
      url: '/webhook',
      method: 'DELETE',
      data: {
        uuid: webhook.uuid,
      },
      headers: {
        Authorization: account.jwt,
      },
    });

    if (error) {
      alert(`Could not delete webhook: ${JSON.stringify(error)}`);
      return;
    }

    removeWebhookItem(webhook);
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
        <Flex sx={{border: '1px solid red'}}>{webhook.uuid}</Flex>
        <Flex sx={{border: '1px solid blue'}} justifyContent={'space-evenly'}>
          <Flex sx={{border: '1px solid green', width: 150}} flexDirection={'column'}>
            <Flex sx={{border: '1px solid red'}}>{webhook.name}</Flex>
            <Flex sx={{border: '1px solid red'}}>{webhook.endpoint}</Flex>
          </Flex>
          <Flex sx={{border: '1px solid red'}}>{webhook.type}</Flex>
          <Flex sx={{border: '1px solid red'}}>{webhook.active}</Flex>
        </Flex>
        <Flex>
          <Box sx={{border: '1px solid orange'}}>
            <TrashOutline
              onClick={async () => await deleteWebhook()}
              style={{cursor: 'hover'}}
              height={'25px'}
              width={'25px'}
            />
          </Box>
          <Switch
            checked={webhook.active}
            backgroundColor={'white'}
            onClick={async () => await toggleWebhookActive(webhook, account)}
          />
        </Flex>
      </Flex>
    </Flex>
  );
};

interface WebhooksViewProps extends HistoryProps {}

interface WebhooksViewState {
  account: Account;
  items: Array<Webhook>;
  showAddWebhookModal: boolean;
  currentWebhook: CurrentWebhookItemProps;
}

class WebhooksView extends React.Component<WebhooksViewProps, WebhooksViewState> {
  constructor(props: WebhooksViewProps) {
    super(props);
    this.state = {
      account: Session.getSession(),
      showAddWebhookModal: false,
      currentWebhook: {
        name: '',
        type: WebhookType.Incoming,
        endpoint: '',
        active: false,
      },
      items: [],
    };

    this.removeWebhookItem = this.removeWebhookItem.bind(this);
    this.updateWebhookItem = this.updateWebhookItem.bind(this);
  }

  async componentDidMount() {
    const {data, error} = await httpRequest({
      url: '/webhook',
      method: 'GET',
      headers: {
        Authorization: this.state.account.jwt,
      },
    });

    if (error) {
      alert(`Failed to fetch webhooks`);
      this.setState({items: []});
      return;
    }

    this.setState({items: data});
  }

  updateWebhookItem(webhook: Webhook) {
    const findWebhook = (wh: Webhook) => wh.uuid == webhook.uuid;
    const index = this.state.items.findIndex(findWebhook);
    const items = this.state.items;
    items[index] = webhook;
    this.setState({items});
  }

  removeWebhookItem(webhook: Webhook) {
    const updated = this.state.items.filter((item) => item.uuid !== webhook.uuid);
    this.setState({items: updated});
  }

  updateCurrentWebhook(key: string, value: string | boolean) {
    this.setState({currentWebhook: {...this.state.currentWebhook, [key]: value}});
  }

  async saveCurrentWebhook() {
    const {data, error} = await httpRequest({
      url: '/webhook',
      method: 'POST',
      data: {
        ...this.state.currentWebhook,
      },
      headers: {
        Authorization: this.state.account.jwt,
      },
    });

    if (error) {
      console.log(`Failed to add webhook: `, error);
      alert(error);
      return null;
    }

    const webhook = Webhook.fromObject(data);
    this.setState({items: [...this.state.items, webhook]}, () => {
      this.setState({showAddWebhookModal: false});
    });

    throw Error;
  }

  renderAddWebhookModal() {
    if (this.state.showAddWebhookModal) {
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
                value={this.state.currentWebhook.name}
                onChange={(e) => this.updateCurrentWebhook('name', e.target.value)}
              />
              <Text sx={{fontSize: 12, color: 'grey', marginTop: 1}}>Some details</Text>
            </Box>
            <Box sx={{border: '1px solid blue', marginBottom: 20, padding: 10}}>
              <Label>Endpoint</Label>
              <Input
                type="text"
                value={this.state.currentWebhook.endpoint}
                onChange={(e) => this.updateCurrentWebhook('endpoint', e.target.value)}
              />
              <Text sx={{fontSize: 12, color: 'grey', marginTop: 1}}>Some details</Text>
            </Box>
            <Box sx={{border: '1px solid blue', marginBottom: 20, padding: 10}}>
              <Label>Type</Label>
              <Select
                id="type"
                name="type"
                defaultValue={WebhookType.Incoming}
                onChange={(e) => {
                  this.updateCurrentWebhook('type', e.target.value);
                }}
              >
                <option key={WebhookType.Incoming}>{WebhookType.Incoming}</option>

                <option key={WebhookType.Outgoing}>{WebhookType.Outgoing}</option>
              </Select>
              <Text sx={{fontSize: 12, color: 'grey', marginTop: 1}}>Some details</Text>
            </Box>
            <Box sx={{border: '1px solid blue', marginBottom: 20, padding: 10}}>
              <Label>Active</Label>
              <Switch
                checked={this.state.currentWebhook.active}
                backgroundColor={'white'}
                onClick={(_e) =>
                  this.updateCurrentWebhook('active', !this.state.currentWebhook.active)
                }
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
                onClick={() => this.setState({showAddWebhookModal: false})}
              >
                Cancel
              </Button>
              <Button
                sx={{width: 100, cursor: 'pointer', position: 'relative', right: 0}}
                onClick={async () => this.saveCurrentWebhook()}
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
    return this.state.items.map((item: Webhook, i: number) => {
      return (
        <WebhookItem
          key={String(i)}
          webhook={item}
          removeWebhookItem={this.removeWebhookItem}
          updateWebhookItem={this.updateWebhookItem}
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
            onClick={() => this.setState({showAddWebhookModal: true})}
          >
            Add
          </Button>
          {this.renderAddWebhookModal()}
          {this.renderFlatList()}
        </Flex>
      </Flex>
    );
  }
}

export default WebhooksView;
