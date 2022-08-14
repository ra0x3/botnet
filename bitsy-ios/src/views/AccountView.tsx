import React from 'react';
import {View, SafeAreaView, StatusBar, Text, TouchableOpacity} from 'react-native';
import {ScrollView} from 'react-native-gesture-handler';
import Ionicons from 'react-native-vector-icons/Ionicons';
import {color} from '../const';
import {NavigationProps} from '../global';

interface AccountListItemProps {
  style?: {[key: string]: number | string};
  title: string;
  onPress: any;
}

const AccountListItem = ({style, title, onPress}: AccountListItemProps) => {
  return (
    <TouchableOpacity onPress={onPress}>
      <View
        style={{
          borderWidth: 1,
          width: 350,
          display: 'flex',
          flexDirection: 'row',
          // justifyContent: 'center',
          alignItems: 'center',
          backgroundColor: color.white,
          padding: 5,
          ...style,
        }}
      >
        <View
          style={{
            borderWidth: 1,
            borderColor: 'green',
            width: '95%',
            height: '100%',
            display: 'flex',
            flexDirection: 'row',
            alignItems: 'center',
            justifyContent: 'center',
          }}
        >
          <View style={{height: '60%', width: '20%', borderWidth: 1}}>
            <Text>Icon</Text>
          </View>

          <View style={{height: '60%', width: '60%', borderWidth: 1}}>
            <Text>{title}</Text>
          </View>

          <View style={{height: '60%', width: '10%', borderWidth: 1}}>
            <TouchableOpacity>
              <Ionicons name="ios-chevron-forward-outline" size={20} color={color.light_grey} />
            </TouchableOpacity>
          </View>
        </View>
      </View>
    </TouchableOpacity>
  );
};

interface AccountViewState {}

interface AccountViewProps extends NavigationProps {}

class AccountView extends React.Component<AccountViewProps, AccountViewState> {
  constructor(props: AccountViewProps) {
    super(props);
    this.state = {};
  }

  render() {
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
              // alignItems: 'center',
            }}
          >
            <ScrollView
              style={{
                display: 'flex',
                borderWidth: 1,
                borderColor: 'red',
                padding: 10,
                width: '100%',
              }}
            >
              <Text style={{fontWeight: 'bold', fontSize: 32, marginTop: 20}}>Account</Text>
              <View
                style={{
                  borderWidth: 1,
                  borderColor: 'blue',
                  display: 'flex',
                  alignItems: 'center',
                  width: '100%',
                }}
              >
                <TouchableOpacity onPress={() => {}}>
                  <View
                    style={{
                      borderWidth: 1,
                      width: 350,
                      display: 'flex',
                      flexDirection: 'row',
                      // justifyContent: 'center',
                      alignItems: 'center',
                      backgroundColor: color.white,
                      padding: 5,
                      height: 125,
                    }}
                  >
                    <View
                      style={{
                        borderWidth: 1,
                        borderColor: 'green',
                        width: '95%',
                        height: '100%',
                        display: 'flex',
                        flexDirection: 'row',
                        alignItems: 'center',
                        justifyContent: 'center',
                      }}
                    >
                      <View
                        style={{
                          height: '60%',
                          width: '20%',
                          borderWidth: 1,
                          borderRadius: 50,
                          display: 'flex',
                          justifyContent: 'center',
                          alignItems: 'center',
                        }}
                      >
                        <Text>Icon</Text>
                      </View>

                      <View
                        style={{
                          height: '60%',
                          width: '60%',
                          borderWidth: 1,
                          display: 'flex',
                          justifyContent: 'center',
                        }}
                      >
                        <Text>Airplane Mode</Text>
                      </View>

                      <View
                        style={{
                          height: '60%',
                          width: '10%',
                          borderWidth: 1,
                          display: 'flex',
                          justifyContent: 'center',
                          alignItems: 'center',
                        }}
                      >
                        <TouchableOpacity>
                          <Ionicons
                            name="ios-chevron-forward-outline"
                            size={20}
                            color={color.light_grey}
                          />
                        </TouchableOpacity>
                      </View>
                    </View>
                  </View>
                </TouchableOpacity>
                <AccountListItem
                  title={'Settings'}
                  onPress={() => this.props.navigation.navigate('Settings')}
                  style={{height: 50}}
                />
                {/* <AccountListItem
                  title={'Permissions'}
                  onPress={() => this.props.navigation.navigate('Permissions')}
                  style={{height: 50, borderBottomRightRadius: 10, borderBottomLeftRadius: 10}}
                /> */}
              </View>
            </ScrollView>
          </View>
        </SafeAreaView>
      </React.Fragment>
    );
  }
}

export default AccountView;
