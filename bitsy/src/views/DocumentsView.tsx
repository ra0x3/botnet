import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {Button, Portal, Dialog, Paragraph} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
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
      <View
        style={{
          borderWidth: 1,
          borderColor: color.light_grey,
          width: '100%',
          height: 100,
        }}
      >
        <Text>{item.name}</Text>
        {/* {this.renderDialogue()} */}
        <Button onPress={() => navigate('FocusedDocument', {item})}>View</Button>
      </View>
    );
  };
}

interface DocumentsViewState {
  items: Array<Document>;
}

interface DocumentsViewProps extends NavigationProps {}

class DocumentsView extends React.Component<DocumentsViewProps, DocumentsViewState> {
  constructor(props: DocumentsViewProps) {
    super(props);
    this.state = {
      items: generateFakeItems(
        new Document(
          '0x123',
          'Chase Saphire Reserve',
          new DocumentBlob('<xml>credit card</xml>'),
          new Account('0x1234', '0x33333', 12345, '', null),
          '0x1234xx',
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
          <View style={{borderWidth: 1, borderColor: 'red', height: 100, width: '100%'}}></View>
          <View
            style={{
              display: 'flex',
              width: '100%',
              borderWidth: 1,
              borderColor: 'blue',
            }}
          >
            <FlatList
              data={this.state.items}
              renderItem={this.renderItem}
              keyExtractor={(item) => item.cid}
            />
          </View>
        </View>
      </SafeAreaView>
    );
  };
}

export default DocumentsView;
