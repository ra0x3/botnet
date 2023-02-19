import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {List, Switch, Appbar} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import {generateFakeItems, httpRequest} from '../utils';
import {ActionState, ErrorMessage, NavigationProps, Option} from '../global';
import Account from '../models/Account';
import Setting from '../models/Setting';
import Session from '../services/Session';

interface SettingsViewItemProps {
  item: Setting;
  toggleSetting: any;
  navigate: any;
}

interface SettingsViewItemState {}

interface FlatListItemProps {
  item: Setting;
}

class SettingsViewItem extends React.Component<SettingsViewItemProps, SettingsViewItemState> {
  constructor(props: SettingsViewItemProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    const {item, navigate, toggleSetting} = this.props;
    return (
      <List.Item
        titleEllipsizeMode={'tail'}
        descriptionEllipsizeMode={'tail'}
        style={{
          borderWidth: 1,
          borderColor: color.light_grey,
          width: '100%',
        }}
        title={item.key}
        description={item.description()}
        right={(props) => {
          return (
            <View
              style={{
                borderWidth: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Switch value={item.value} onValueChange={toggleSetting} />
            </View>
          );
        }}
      />
    );
  };
}

interface SettingsViewState {
  items: Array<Setting>;
  query: string;
  actionState: ActionState;
  error: Option<ErrorMessage>;
  account: Option<Account>;
}

interface SettingsViewProps extends NavigationProps {}

class SettingsView extends React.Component<SettingsViewProps, SettingsViewState> {
  constructor(props: SettingsViewProps) {
    super(props);
    this.state = {
      query: '',
      error: null,
      account: null,
      actionState: ActionState.none,
      items: [],
    };

    this.navigate = this.navigate.bind(this);
    this.toggleSetting = this.toggleSetting.bind(this);
  }

  toggleSetting = (key: string, value: boolean) => {};

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
  };

  componentDidMount = async () => {
    const account = await Session.getSession();

    this.setState({actionState: ActionState.pending, account}, async () => {
      const {data, error} = await httpRequest({
        url: '/account/setting',
        method: 'GET',
        headers: {
          Authorization: this.state?.account!.jwt,
        },
      });

      if (error) {
        this.setState({error, actionState: ActionState.error});
        return;
      }

      this.setState({
        items: data.map((item: any, i: number) => Setting.fromObject(item)),
        actionState: ActionState.success,
      });
    });
  };

  renderItem = ({item}: FlatListItemProps) => {
    return (
      <SettingsViewItem
        toggleSetting={this.toggleSetting}
        navigate={this.navigate}
        item={item}
        key={String(item.key)}
      />
    );
  };

  onSearchChange = (query: string) => {
    this.setState({query});
  };

  filteredResults() {
    if (this.state.query === '') {
      return this.state.items;
    } else {
      const query = this.state.query.toLowerCase();
      return this.state.items.filter((item: Setting, i: number) => {
        return (
          item.key.toLowerCase().startsWith(query) ||
          item.description().toLowerCase().includes(query)
        );
      });
    }
  }

  render = () => {
    return (
      <SafeAreaView>
        <StatusBar />
        <View
          style={{
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <View style={{borderWidth: 1, borderColor: 'red', height: 100, width: '100%'}}>
            <Appbar.Header>
              <Appbar.Content title="Settings" subtitle={'Manage your account settings'} />
            </Appbar.Header>
            <SearchBar onChangeText={this.onSearchChange} query={this.state.query} />
          </View>
          <View
            style={{
              display: 'flex',
              width: '100%',
              height: '100%',
              borderWidth: 1,
              borderColor: 'blue',
            }}
          >
            <FlatList
              data={this.filteredResults()}
              renderItem={this.renderItem}
              keyExtractor={(item) => item.key}
            />
          </View>
        </View>
      </SafeAreaView>
    );
  };
}

export default SettingsView;
