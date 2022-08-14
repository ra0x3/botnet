import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import {FlatList} from 'react-native-gesture-handler';
import {Button} from 'react-native-paper';
import {color} from '../const';
import {ActionState, NavigationProps, Option} from '../global';
import {generateFakeItems} from '../utils';
import ThirdParty from '../models/ThirdParty';
import Account from '../models/Account';
import Document from '../models/Document';
import AccessRequest, {AccessRequestStatus} from '../models/AccessRequest';
import DocumentBlob from '../models/DocumentBlob';
import AccessRequestsViewItem, {FlatListItemProps} from '../components/AccessRequestListItem';

interface FocusDocumentViewState {
  document: Option<Document>;
  actionState: ActionState;
}

interface FocusDocumentViewProps extends NavigationProps {}

class FocusDocumentView extends React.Component<FocusDocumentViewProps, FocusDocumentViewState> {
  constructor(props: FocusDocumentViewProps) {
    super(props);
    this.state = {
      document: null,
      actionState: ActionState.none,
    };
    this.navigate = this.navigate.bind(this);
  }

  componentDidMount = async () => {};

  renderAccessRequestHistoryItem = ({item}: FlatListItemProps) => {
    return <AccessRequestsViewItem navigate={this.navigate} item={item} key={String(item.uuid)} />;
  };

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
  };

  render() {
    return (
      <SafeAreaView>
        <StatusBar />
        <View
          style={{
            height: '100%',
            width: '100%',
            display: 'flex',
          }}
        >
          <View
            style={{justifyContent: 'center', alignItems: 'center', display: 'flex', width: '100%'}}
          >
            <View style={{borderWidth: 1, borderColor: 'red', width: '100%'}}>
              <View
                style={{
                  borderWidth: 1,
                  height: 250,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  width: '100%',
                  justifyContent: 'center',
                }}
              >
                <View
                  style={{
                    borderWidth: 1,
                    width: '90%',
                    height: '90%',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                  }}
                >
                  <Text>Image</Text>
                </View>
              </View>

              <View
                style={{
                  borderWidth: 1,
                  height: 250,
                  marginTop: 20,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'row',
                  width: '100%',
                }}
              >
                <View style={{borderWidth: 1, borderColor: 'green', width: '50%', height: 200}}>
                  <View
                    style={{
                      borderWidth: 1,
                      height: 100,
                      width: '100%',
                      display: 'flex',
                      justifyContent: 'center',
                      alignItems: 'center',
                    }}
                  >
                    <View
                      style={{
                        borderWidth: 1,
                        width: '90%',
                        height: '85%',
                        backgroundColor: color.white,
                        borderRadius: 10,
                        display: 'flex',
                        // justifyContent: 'center',
                        alignItems: 'center',
                        borderColor: color.light_grey,
                      }}
                    >
                      <Text style={{fontSize: 14, marginTop: 10}}>Access Requests</Text>
                      <Text style={{fontSize: 18, marginTop: 10, fontWeight: 'bold'}}>16</Text>
                    </View>
                  </View>
                  <View
                    style={{
                      borderWidth: 1,
                      height: 100,
                      width: '100%',
                      display: 'flex',
                      justifyContent: 'center',
                      alignItems: 'center',
                    }}
                  >
                    <View
                      style={{
                        borderWidth: 1,
                        width: '90%',
                        height: '85%',
                        backgroundColor: color.white,
                        borderRadius: 10,
                      }}
                    ></View>
                  </View>
                </View>
                <View
                  style={{
                    borderWidth: 1,
                    borderColor: 'red',
                    width: '50%',
                    height: 200,
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                  }}
                >
                  <View
                    style={{
                      borderWidth: 1,
                      width: '90%',
                      height: '100%',
                      backgroundColor: color.white,
                      borderRadius: 10,
                      display: 'flex',
                      alignItems: 'center',
                    }}
                  >
                    <Text style={{marginTop: 10}}>Last accessed</Text>
                    <Text style={{marginTop: 10, color: color.grey}}>
                      On 14 days ago by Taboola Inc.
                    </Text>
                    <Button mode="outlined" style={{width: 150, height: 40, marginTop: 30}}>
                      View Request
                    </Button>
                  </View>
                </View>
              </View>
              <View
                style={{
                  borderWidth: 1,
                  height: 250,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  width: '100%',
                  justifyContent: 'center',
                }}
              >
                <View
                  style={{
                    borderWidth: 1,
                    width: '95%',
                    height: '90%',
                    backgroundColor: color.white,
                    borderRadius: 10,
                    display: 'flex',
                    alignItems: 'center',
                  }}
                >
                  <Text>Access Request History</Text>
                  <FlatList
                    style={{width: '100%'}}
                    renderItem={this.renderAccessRequestHistoryItem}
                    data={generateFakeItems(
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
                    )}
                  />
                </View>
              </View>
            </View>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default FocusDocumentView;
