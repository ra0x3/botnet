import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, TouchableOpacity} from 'react-native';
import {List} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import FeedItem, {FeedItemType} from './../models/FeedItem';
import {generateFakeItems} from '../utils';
import {NavigationProps} from '../global';

interface FeedViewItemProps {
  item: FeedItem;
  navigate: any;
}

interface FeedViewItemState {}

interface FlatListItemProps {
  item: FeedItem;
}

class FeedViewItem extends React.Component<FeedViewItemProps, FeedViewItemState> {
  constructor(props: FeedViewItemProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    const {item, navigate} = this.props;
    return (
      <List.Item
        titleEllipsizeMode={'tail'}
        descriptionEllipsizeMode={'tail'}
        style={{
          borderWidth: 1,
          borderColor: color.light_grey,
          width: '100%',
        }}
        title={item.title}
        description={item.subtitle}
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
              <Ionicons name={'ios-chevron-forward-outline'} size={25} color={color.light_grey} />
            </View>
          );
        }}
        onPress={() => {
          switch (item.type) {
            case FeedItemType.AccessRequest:
              navigate('FocusAccessRequest');
              break;
            case FeedItemType.Document:
              navigate('FocusDocument');
              break;
            default:
              return;
          }
        }}
      />
    );
  };
}

interface FeedViewState {
  items: Array<FeedItem>;
  query: string;
}

interface FeedViewProps extends NavigationProps {}

class FeedView extends React.Component<FeedViewProps, FeedViewState> {
  constructor(props: FeedViewProps) {
    super(props);
    this.state = {
      query: '',
      items: generateFakeItems(
        new FeedItem(
          'Foo',
          'Bar this is a subheading description',
          'This is the text related to the item but it is shortedned so that it is not so long.',
          FeedItemType.AccessRequest,
          1659572994892,
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
    return <FeedViewItem navigate={this.navigate} item={item} key={String(item.id())} />;
  };

  onSearchChange = (query: string) => {
    this.setState({query});
  };

  filteredResults() {
    if (this.state.query === '') {
      return this.state.items;
    } else {
      const query = this.state.query.toLowerCase();
      return this.state.items.filter((item: FeedItem, i: number) => {
        return (
          item.title.toLowerCase().startsWith(query) ||
          item.subtitle.toLowerCase().startsWith(query)
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
              keyExtractor={(item) => item.id()}
            />
          </View>
        </View>
      </SafeAreaView>
    );
  };
}

export default FeedView;
