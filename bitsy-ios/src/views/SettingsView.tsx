import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {List, Switch} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import {generateFakeItems} from '../utils';
import {NavigationProps} from '../global';
import Account from '../models/Account';
import Setting from '../models/Setting';

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
}

interface SettingsViewProps extends NavigationProps {}

class SettingsView extends React.Component<SettingsViewProps, SettingsViewState> {
  constructor(props: SettingsViewProps) {
    super(props);
    this.state = {
      query: '',
      items: generateFakeItems(
        new Setting(
          'BitsyVaultDeletegation',
          true,
          new Account('0x123', '0x3333', 'mypassword', 123, '', null),
        ),
        20,
      ),
    };

    this.navigate = this.navigate.bind(this);
    this.toggleSetting = this.toggleSetting.bind(this);
  }

  toggleSetting = (key: string, value: boolean) => {};

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
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
            <SearchBar onChangeText={this.onSearchChange} query={this.state.query} />
          </View>
          <View
            style={{
              display: 'flex',
              width: '100%',
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
