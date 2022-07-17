import React from 'react';
import {SearchCircleOutline} from 'react-ionicons';
import {Flex, Box, Text, Button} from 'rebass';
import {Input, Label, Switch} from '@rebass/forms';
import Setting from './../models/Setting';
import TopNavigation from '../components/TopNavigation';
import Session from '../services/Session';
import Account from '../models/Account';
import {httpRequest} from '../utils';
import SettingsMetadata from '../etc/settings.json';
import inflection from 'inflection';

interface SettingItemState {}

interface SettingItemProps {
  setting: Setting;
  metadata: {[key: string]: string};
  account: Account;
  updateSettingItem: any;
}

const SettingItem = ({setting, metadata, account, updateSettingItem}: SettingItemProps) => {
  const toggleSettingActive = async (setting: Setting, acount: Account) => {
    const {data, error} = await httpRequest({
      url: '/setting',
      method: 'PUT',
      data: {
        key: setting.key,
      },
      headers: {
        Authorization: account.jwt,
      },
    });

    if (error) {
      alert(`Could not update setting.`);
      console.error(`Could not update setting: `, error);
    }

    updateSettingItem(data);
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
      flexDirection={'column'}
    >
      <Flex sx={{border: '1px solid red', width: '100%'}} flexDirection={'column'}>
        <Text>{setting.key}</Text>
        <Text>{metadata.description}</Text>
      </Flex>
      <Switch
        checked={setting.value}
        backgroundColor={'white'}
        onClick={async () => await toggleSettingActive(setting, account)}
      />
      <Text>{inflection.dasherize(setting.key)}</Text>
    </Flex>
  );
};

interface SettingsViewProps extends HistoryProps {}

interface SettingsViewState {
  items: Array<Setting>;
  account: Account;
  query: string;
}

class SettingsView extends React.Component<SettingsViewProps, SettingsViewState> {
  constructor(props: SettingsViewProps) {
    super(props);
    this.state = {
      items: [],
      account: Session.getSession(),
      query: '',
    };

    this.updateSettingItem = this.updateSettingItem.bind(this);
  }

  async componentDidMount() {
    const {data, error} = await httpRequest({
      url: '/setting',
      method: 'GET',
      headers: {
        Authorization: this.state.account.jwt,
      },
    });

    if (error) {
      alert(`Could not get settings.`);
      console.error(`Could not get settings `, error);
      return;
    }

    this.setState({items: data.sort((x: Setting, y: Setting) => x.key > y.key)});
  }

  updateSettingItem(setting: Setting) {
    const findWebhook = (s: Setting) => s.key == setting.key;
    const index = this.state.items.findIndex(findWebhook);
    const items = this.state.items;
    items[index] = setting;
    this.setState({items});
  }

  renderFlatList() {
    return this.fuzzySearch().map((item: Setting, i: number) => {
      const metadata: {[key: string]: string} = (SettingsMetadata as any)[item.key];
      return (
        <SettingItem
          updateSettingItem={this.updateSettingItem}
          account={this.state.account}
          key={String(i)}
          setting={item}
          metadata={metadata}
        />
      );
    });
  }

  fuzzySearch() {
    if (this.state.query === '') {
      return this.state.items;
    }

    return this.state.items.filter((item: Setting) =>
      item.key.toLowerCase().startsWith(this.state.query.toLowerCase()),
    );
  }

  render() {
    return (
      <Flex
        justifyContent={'center'}
        flexDirection={'column'}
        sx={{width: '100%'}}
        alignItems={'center'}
      >
        <TopNavigation />
        <Flex
          sx={{border: '1px solid red', width: 1000}}
          justifyContent={'center'}
          alignItems={'center'}
        >
          <Flex
            sx={{border: '1px solid blue', width: '100%'}}
            justifyContent={'center'}
            alignItems={'center'}
            flexDirection={'column'}
          >
            <Flex
              sx={{border: '1px solid red', width: '75%'}}
              justifyContent={'center'}
              alignItems={'center'}
            >
              <Flex>
                <Box sx={{border: ' 1px solid green', width: 100, heigh: 100}}>
                  <SearchCircleOutline style={{cursor: 'pointer'}} />
                </Box>
                <Input
                  value={this.state.query}
                  onChange={(e) => this.setState({query: e.target.value})}
                />
              </Flex>
            </Flex>
            <Flex sx={{border: '1px solid black', width: '100%'}} justifyContent={'center'}>
              <Flex
                alignItems={'center'}
                flexDirection={'column'}
                sx={{border: '1px solid black', width: 600, height: 800, padding: 10}}
              >
                {this.renderFlatList()}
              </Flex>
            </Flex>
          </Flex>
        </Flex>
      </Flex>
    );
  }
}

export default SettingsView;
