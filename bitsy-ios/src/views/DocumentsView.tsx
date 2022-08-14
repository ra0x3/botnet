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
  TextInput,
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

interface DocumentsViewItemState {}

interface FlatListItemProps {
  item: Document;
}

class DocumentsViewItem extends React.Component<DocumentsViewItemProps, DocumentsViewItemState> {
  constructor(props: DocumentsViewItemProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    const {item, navigate} = this.props;
    return (
      <View
        style={{
          borderWidth: 1,
          borderColor: color.light_grey,
          width: 375,
          height: 100,
        }}
      >
        <View
          style={{
            borderWidth: 1,
            borderColor: 'yellow',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            flexDirection: 'row',
            height: '100%',
            width: '100%',
          }}
        >
          <View
            style={{
              borderWidth: 1,
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              width: 50,
              height: 50,
              borderRadius: 50,
            }}
          >
            <Text>Image</Text>
          </View>
          <View
            style={{
              borderWidth: 1,
              marginLeft: 10,
              height: '100%',
              width: 275,
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
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
              <Text>Added on {item.created_at}</Text>
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
                style={{width: 75, borderWidth: 1, height: 30, padding: 0}}
                labelStyle={{fontSize: 10}}
                mode={'outlined'}
                onPress={() => navigate('FocusedDocument', {item})}
              >
                View
              </Button>
            </View>
          </View>
        </View>
      </View>
    );
  };
}

interface BankCardForm {
  type: DocumentType;
  name: string;
  number: string;
  securityCode: string;
  expiry: string; // MM-YYYY
  zipCode: string;
}

interface BankAccountForm {
  type: DocumentType;
  name: string;
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
            <View style={{borderWidth: 1, borderColor: 'orange', width: '95%', height: '95%'}}>
              <TextInput
                // @ts-ignore
                onChangeText={(text) => this.setState({form: {...this.state.form, name: text}})}
                mode="outlined"
                style={{width: '100%', height: 40}}
                label="Name on account"
                value={(this.state.form as BankAccountForm).name}
              />
              <TextInput
                onChangeText={(text) =>
                  // @ts-ignore
                  this.setState({form: {...this.state.form, routingNumber: text}})
                }
                value={(this.state.form as BankAccountForm).routingNumber}
                label="Routing Number"
                mode="outlined"
                style={{width: '100%', height: 40}}
              />
              <TextInput
                label="Account Number"
                mode="outlined"
                style={{width: '100%', height: 40}}
                value={(this.state.form as BankAccountForm).accountNumber}
                onChangeText={(text) =>
                  // @ts-ignore
                  this.setState({form: {...this.state.form, accountNumber: text}})
                }
              />
            </View>
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
            <View style={{borderWidth: 1, borderColor: 'orange', width: '95%', height: '95%'}}>
              <TextInput
                mode="outlined"
                style={{width: '100%', height: 40}}
                label="Name on card"
                value={(this.state.form as BankCardForm).name}
                onChangeText={(text) =>
                  // @ts-ignore
                  this.setState({form: {...this.state.form, name: text}})
                }
              />
              <TextInput
                value={(this.state.form as BankCardForm).number}
                label="Card Number"
                mode="outlined"
                style={{width: '100%', height: 40}}
                onChangeText={(text) =>
                  // @ts-ignore
                  this.setState({form: {...this.state.form, number: text}})
                }
              />
              <View
                style={{
                  display: 'flex',
                  flexDirection: 'row',
                  borderWidth: 1,
                  width: '100%',
                  justifyContent: 'space-between',
                  alignItems: 'center',
                }}
              >
                <TextInput
                  mode="outlined"
                  style={{width: 100, height: 40}}
                  label="Expiration Date"
                  value={(this.state.form as BankCardForm).expiry}
                  onChangeText={(text) =>
                    // @ts-ignore
                    this.setState({form: {...this.state.form, expiry: text}})
                  }
                />
                <TextInput
                  mode="outlined"
                  style={{width: 100, height: 40}}
                  label="Security Code"
                  value={(this.state.form as BankCardForm).securityCode}
                  onChangeText={(text) =>
                    // @ts-ignore
                    this.setState({form: {...this.state.form, securityCode: text}})
                  }
                />
              </View>
              <TextInput
                label="Zip Code"
                mode="outlined"
                style={{width: '100%', height: 40}}
                value={(this.state.form as BankCardForm).zipCode}
                onChangeText={(text) =>
                  // @ts-ignore
                  this.setState({form: {...this.state.form, zipCode: text}})
                }
              />
            </View>
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
            <TextInput
              label="Document"
              mode="outlined"
              multiline={true}
              value={(this.state.form as BasicForm).text}
              onChangeText={(text) => this.setState({form: {text, type: DocumentType.Basic}})}
              style={{width: '100%', height: 250}}
            />
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
            marginTop: 10,
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
            height: 450,
            position: 'absolute',
            zIndex: 10,
            padding: 10,
          }}
        >
          <Text style={{textAlign: 'center', marginBottom: 10}}>Add a document</Text>
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
                        name: '',
                        number: '',
                        securityCode: '',
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
                        name: '',
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
          <View
            style={{
              borderWidth: 1,
              width: '100%',
              display: 'flex',
              justifyContent: 'space-evenly',
              alignItems: 'center',
              flexDirection: 'row',
            }}
          >
            <Button mode="outlined" style={{marginTop: 20}} onPress={() => {}}>
              Save
            </Button>
            <Button
              mode="outlined"
              style={{marginTop: 10}}
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
