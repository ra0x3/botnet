import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text, TouchableOpacity} from 'react-native';
import {Button, Portal, Dialog, Paragraph, Appbar} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import {generateFakeItems} from '../utils';
import {NavigationProps} from '../global';
import Account from '../models/Account';
import Document from '../models/Document';
import DocumentBlob from '../models/DocumentBlob';

interface DocumentsViewItemProps {
  item: Document;
  navigate: any;
}

interface DocumentsViewItemState {
  renderDialogue: boolean;
}

interface FlatListItemProps {
  item: Document;
}

class DocumentsViewItem extends React.Component<DocumentsViewItemProps, DocumentsViewItemState> {
  constructor(props: DocumentsViewItemProps) {
    super(props);
    this.state = {
      renderDialogue: false,
    };
  }

  renderDialogue = () => {
    const {item} = this.props;
    if (this.state.renderDialogue) {
      return (
        <Portal>
          <Dialog
            visible={this.state.renderDialogue}
            onDismiss={() => this.setState({renderDialogue: false})}
          >
            <Dialog.Title>This is a title</Dialog.Title>
            <Dialog.Content>
              <Paragraph>This is simple dialog</Paragraph>
              <Paragraph>{item.blob.data}</Paragraph>
            </Dialog.Content>
            <Button onPress={() => this.setState({renderDialogue: false})}>Close</Button>
          </Dialog>
        </Portal>
      );
    }
  };

  render = () => {
    const {item, navigate} = this.props;
    return (
      <TouchableOpacity onPress={() => navigate('FocusedDocument', {item})}>
        <View
          style={{
            borderWidth: 1,
            borderColor: color.light_grey,
            width: '100%',
            height: 100,
          }}
        >
          <View
            style={{
              borderWidth: 1,
              borderColor: 'red',
              width: '100%',
              display: 'flex',
              justifyContent: 'center',
            }}
          >
            <Text>{item.name}</Text>
            <Text>{item.created_at}</Text>
          </View>
          <View
            style={{
              borderWidth: 1,
              borderColor: 'blue',
              width: '100%',
              display: 'flex',
              flexDirection: 'row',
              justifyContent: 'space-between',
            }}
          >
            <Button
              style={{width: 75, borderWidth: 1, height: 30}}
              mode={'outlined'}
              onPress={() => navigate('FocusedDocument', {item})}
            >
              View
            </Button>
          </View>
        </View>
      </TouchableOpacity>
    );
  };
}

interface DocumentsViewState {
  items: Array<Document>;
  query: string;
}

interface DocumentsViewProps extends NavigationProps {}

class DocumentsView extends React.Component<DocumentsViewProps, DocumentsViewState> {
  constructor(props: DocumentsViewProps) {
    super(props);
    this.state = {
      query: '',
      items: generateFakeItems(
        new Document(
          '0x123',
          'Chase Saphire Reserve',
          new DocumentBlob('<xml>credit card</xml>'),
          new Account('0x1234', '0x33333', 'password', 12345, '', null),
          '0x1234xx',
          12345,
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
    return <DocumentsViewItem navigate={this.navigate} item={item} key={String(item.cid)} />;
  };

  onSearchChange = (query: string) => {
    this.setState({query});
  };

  filteredResults() {
    if (this.state.query === '') {
      return this.state.items;
    } else {
      const query = this.state.query.toLowerCase();
      return this.state.items.filter((item: Document, i: number) => {
        return item.name.toLowerCase().startsWith(query);
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
                <Appbar.Content title="Documents" subtitle={'Manage your documents'} />
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
                marginTop: 20,
              }}
            >
              <FlatList
                data={this.filteredResults()}
                renderItem={this.renderItem}
                keyExtractor={(item) => item.cid}
              />
            </View>
          </View>
        </SafeAreaView>
      </React.Fragment>
    );
  };
}

export default DocumentsView;
