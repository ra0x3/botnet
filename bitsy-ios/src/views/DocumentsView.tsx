import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text, TouchableOpacity} from 'react-native';
import {
  Button,
  Portal,
  Dialog,
  Paragraph,
  Appbar,
  AnimatedFAB,
  Menu,
  Divider,
} from 'react-native-paper';
import Dropdown from 'react-native-paper-dropdown';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import SearchBar from '../components/SearchBar';
import {generateFakeItems} from '../utils';
import {NavigationProps, Option} from '../global';
import Account from '../models/Account';
import Document, {DocumentType} from '../models/Document';
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
            width: 350,
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

interface BankCardForm {
  type: DocumentType;
  firstName: string;
  lastName: string;
  address: string;
  number: string;
  ccv: string;
  expiry: string; // MM-YYYY
  zipCode: string;
}

interface BankAccountForm {
  type: DocumentType;
  firstName: string;
  lastName: string;
  accountNumber: string;
  routingNumber: string;
}

interface BasicForm {
  type: DocumentType;
  text: string;
}

type DocumentForm = BankCardForm | BankAccountForm | BasicForm;

interface DocumentsViewState {
  items: Array<Document>;
  query: string;
  extendedAddButton: boolean;
  renderAddDocumentMenu: boolean;
  showrenderAddDocumentMenuDropdown: boolean;
  showrenderAddDocumentMenuDropdownValue: DocumentType;
  showrenderAddDocumentMenuForm: Option<DocumentType>;
  form: Option<DocumentForm>;
}

interface DocumentsViewProps extends NavigationProps {}

class DocumentsView extends React.Component<DocumentsViewProps, DocumentsViewState> {
  constructor(props: DocumentsViewProps) {
    super(props);
    this.state = {
      query: '',
      extendedAddButton: false,
      renderAddDocumentMenu: false,
      showrenderAddDocumentMenuDropdown: false,
      showrenderAddDocumentMenuDropdownValue: DocumentType.Basic,
      form: null,
      showrenderAddDocumentMenuForm: null,
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

  filteredResults = () => {
    if (this.state.query === '') {
      return this.state.items;
    } else {
      const query = this.state.query.toLowerCase();
      return this.state.items.filter((item: Document, i: number) => {
        return item.name.toLowerCase().startsWith(query);
      });
    }
  };

  renderAddDocumentMenuFormInnerContent = () => {
    switch (this.state.showrenderAddDocumentMenuForm) {
      case DocumentType.BankAccount:
        return (
          <View
            style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
            }}
          >
            <Text>Bank Account</Text>
          </View>
        );
      case DocumentType.BankCard:
        return (
          <View
            style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
            }}
          >
            <Text>Bank Card</Text>
          </View>
        );
      case DocumentType.Basic:
        return (
          <View
            style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
            }}
          >
            <Text>Basic</Text>
          </View>
        );
    }
  };

  renderAddDocumentMenuForm = () => {
    if (this.state.showrenderAddDocumentMenuForm) {
      return (
        <View
          style={{
            borderWidth: 1,
            borderColor: 'green',
            width: '100%',
            height: 250,
          }}
        >
          {this.renderAddDocumentMenuFormInnerContent()}
        </View>
      );
    }
  };

  renderAddDocumentMenu = () => {
    if (this.state.renderAddDocumentMenu) {
      return (
        <View
          style={{
            borderWidth: 1,
            backgroundColor: color.white,
            width: 300,
            height: 400,
            position: 'absolute',
            zIndex: 10,
            padding: 10,
          }}
        >
          <Dropdown
            label="Document Type"
            mode="outlined"
            value={this.state.showrenderAddDocumentMenuDropdownValue}
            setValue={(text) => {
              this.setState({showrenderAddDocumentMenuDropdownValue: text}, () => {
                switch (this.state.showrenderAddDocumentMenuDropdownValue) {
                  case DocumentType.BankCard:
                    this.setState({
                      form: {
                        type: DocumentType.BankCard,
                        firstName: '',
                        lastName: '',
                        address: '',
                        number: '',
                        ccv: '',
                        expiry: '',
                        zipCode: '',
                      },
                      showrenderAddDocumentMenuForm: DocumentType.BankCard,
                    });
                    break;
                  case DocumentType.BankAccount:
                    this.setState({
                      form: {
                        type: DocumentType.BankAccount,
                        firstName: '',
                        lastName: '',
                        accountNumber: '',
                        routingNumber: '',
                      },
                      showrenderAddDocumentMenuForm: DocumentType.BankAccount,
                    });
                    break;
                  case DocumentType.Basic:
                    this.setState({
                      form: {
                        type: DocumentType.Basic,
                        text: '',
                      },
                      showrenderAddDocumentMenuForm: DocumentType.Basic,
                    });
                    break;
                }
              });
            }}
            list={[
              {
                value: DocumentType.Basic,
                label: DocumentType.Basic,
              },
              {
                value: DocumentType.BankCard,
                label: DocumentType.BankCard,
              },
              {
                value: DocumentType.BankAccount,
                label: DocumentType.BankAccount,
              },
            ]}
            inputProps={{right: <Ionicons name={'ios-add-outline'} size={20} />}}
            visible={this.state.showrenderAddDocumentMenuDropdown}
            showDropDown={() => this.setState({showrenderAddDocumentMenuDropdown: true})}
            onDismiss={() => this.setState({showrenderAddDocumentMenuDropdown: false})}
            dropDownStyle={{
              width: '100%',
            }}
          />
          {this.renderAddDocumentMenuForm()}
          <Button
            onPress={() =>
              this.setState({
                renderAddDocumentMenu: false,
                showrenderAddDocumentMenuDropdown: false,
              })
            }
          >
            Close
          </Button>
        </View>
      );
    }
  };

  onScroll = ({nativeEvent}: any) => {
    const currentScrollPosition = Math.floor(nativeEvent?.contentOffset?.y) ?? 0;
    this.setState({extendedAddButton: currentScrollPosition <= 0});
  };

  render = () => {
    return (
      <React.Fragment>
        <SafeAreaView style={{flex: 0, backgroundColor: color.cobalt}} />
        <SafeAreaView>
          <StatusBar />
          <View
            style={{
              height: '100%',
              width: '100%',
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
                  onPress={() => this.setState({renderAddDocumentMenu: true})}
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
                position: 'relative',
                alignItems: 'center',
              }}
            >
              {this.renderAddDocumentMenu()}
              <FlatList
                data={this.filteredResults()}
                onScroll={this.onScroll}
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
