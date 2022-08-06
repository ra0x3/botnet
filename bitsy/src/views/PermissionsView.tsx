import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {List, Switch, Appbar} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import {generateFakeItems, booleanify} from '../utils';
import {NavigationProps} from '../global';
import Account from '../models/Account';
import Document from '../models/Document';
import DocumentBlob from '../models/DocumentBlob';
import Permission from '../models/Permission';
import ThirdParty from '../models/ThirdParty';

interface PermissionsViewItemProps {
  item: Permission;
  togglePermission: any;
  navigate: any;
}

interface PermissionsViewItemState {}

interface FlatListItemProps {
  item: Permission;
}

class PermissionsViewItem extends React.Component<
  PermissionsViewItemProps,
  PermissionsViewItemState
> {
  constructor(props: PermissionsViewItemProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    const {item, navigate, togglePermission} = this.props;
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
        description={'Some permission description'}
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
              <Switch value={booleanify(item.value)} onValueChange={togglePermission} />
            </View>
          );
        }}
        onPress={() => navigate('FocusAccessRequest')}
      />
    );
  };
}

interface PermissionsViewState {
  items: Array<Permission>;
  query: string;
}

interface PermissionsViewProps extends NavigationProps {}

class PermissionsView extends React.Component<PermissionsViewProps, PermissionsViewState> {
  constructor(props: PermissionsViewProps) {
    super(props);
    this.state = {
      query: '',
      items: generateFakeItems(
        new Permission(
          '12345',
          'SomePermissionKey',
          new Document(
            '0x0123',
            'Chase Saphire 1',
            new DocumentBlob('<xml>credit card info</xml>'),
            new Account('', '0x001', 'password', 123, '', null),
            '0x0000',
            123,
          ),
          1,
          new Account('', '0x001', 'password', 123, '', null),
          new ThirdParty('5432', 'Taboola'),
          123,
          456,
        ),
        20,
      ),
    };

    this.navigate = this.navigate.bind(this);
    this.togglePermission = this.togglePermission.bind(this);
  }

  togglePermission = (key: string, value: boolean) => {};

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
  };

  renderItem = ({item}: FlatListItemProps) => {
    return (
      <PermissionsViewItem
        togglePermission={this.togglePermission}
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
      return this.state.items.filter((item: Permission, i: number) => {
        return item.key.toLowerCase().startsWith(query);
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
              <Appbar.Content title="Permission" subtitle={'Manage your permissions'} />
              <Appbar.Action
                style={{borderWidth: 1, height: 50, width: 50}}
                icon={() => (
                  <Ionicons name="ios-add-circle-outline" size={25} color={color.white} />
                )}
                onPress={() => {}}
              />
            </Appbar.Header>
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

export default PermissionsView;
