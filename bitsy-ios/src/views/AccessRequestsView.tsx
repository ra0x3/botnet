import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {List, Appbar} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import AccessRequest, {AccessRequestStatus} from './../models/AccessRequest';
import SearchBar from './../components/SearchBar';
import {generateFakeItems} from '../utils';
import {NavigationProps} from '../global';
import ThirdParty from '../models/ThirdParty';
import Account from '../models/Account';
import Document from '../models/Document';
import DocumentBlob from '../models/DocumentBlob';
import AccessRequestsViewItem, {FlatListItemProps} from '../components/AccessRequestListItem';

interface AccessRequestsViewState {
  items: Array<AccessRequest>;
  query: string;
}

interface AccessRequestsViewProps extends NavigationProps {}

class AccessRequestsView extends React.Component<AccessRequestsViewProps, AccessRequestsViewState> {
  constructor(props: AccessRequestsViewProps) {
    super(props);
    this.state = {
      query: '',
      items: generateFakeItems(
        new AccessRequest(
          '123456',
          new ThirdParty('5432', 'Taboola'),
          new Account('', '0x001', 'password', 123, '', null),
          AccessRequestStatus.Pending,
          new Document(
            '0x0123',
            'Chase Saphire 1',
            new DocumentBlob('<xml>credit card info</xml>'),
            new Account('', '0x001', 'password', 123, '', null),
            '0x0000',
            123,
          ),
          'https://duckduckgo.com',
          {q: 'duckduckgo search'},
          new Date().getTime(),
          new Date().getTime() + 60 * 60 * 24,
        ),
        20,
      ),
    };

    this.navigate = this.navigate.bind(this);
  }

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
  };

  renderItem = ({item}: FlatListItemProps) => {
    return <AccessRequestsViewItem navigate={this.navigate} item={item} key={String(item.uuid)} />;
  };

  onSearchChange = (query: string) => {
    this.setState({query});
  };

  filteredResults() {
    if (this.state.query === '') {
      return this.state.items;
    } else {
      const query = this.state.query.toLowerCase();
      return this.state.items.filter((item: AccessRequest, i: number) => {
        return (
          item.third_party.name.toLowerCase().startsWith(query) ||
          item.document.name.includes(query)
        );
      });
    }
  }

  render = () => {
    return (
      <React.Fragment>
        <SafeAreaView style={{flex: 0, backgroundColor: color.cobalt}} />
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
                <Appbar.Content title="Access Requests" subtitle={'Manage your access requests'} />
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
                keyExtractor={(item) => item.uuid}
              />
            </View>
          </View>
        </SafeAreaView>
      </React.Fragment>
    );
  };
}

export default AccessRequestsView;