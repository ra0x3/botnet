import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import {ScrollView} from 'react-native-gesture-handler';
import {color} from '../const';
import {NavigationProps} from '../global';

interface AccountListItemProps {
  style?: {[key: string]: number | string};
  title: string;
  onPress: any;
}

const AccountListItem = ({style, title, onPress}: AccountListItemProps) => {
  return (
    <View
      style={{
        borderWidth: 1,
        height: 40,
        width: '100%',
        display: 'flex',
        flexDirection: 'row',
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: color.white,
        ...style,
      }}
    >
      <View style={{height: 30, width: 30, borderWidth: 1, padding: 5}}>
        <Text>Icon</Text>
      </View>

      <View style={{height: 30, width: 250, borderWidth: 1, padding: 5}}>
        <Text>{title}</Text>
      </View>

      <View style={{height: 30, width: 30, borderWidth: 1, padding: 5}}>
        <Text>Chevron</Text>
      </View>
    </View>
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
      <SafeAreaView>
        <StatusBar />
        <View
          style={{
            height: '100%',
            display: 'flex',
            // alignItems: 'center',
          }}
        >
          <ScrollView style={{display: 'flex', borderWidth: 1, borderColor: 'red', padding: 20}}>
            <Text style={{fontWeight: 'bold', fontSize: 32, marginTop: 20}}>Account</Text>
            <View
              style={{borderWidth: 1, borderColor: 'blue', display: 'flex', alignItems: 'center'}}
            >
              <AccountListItem
                title={'Airplane Mode'}
                onPress={() => {}}
                style={{height: 100, borderRadius: 10}}
              />
              <AccountListItem
                title={'Notifications'}
                onPress={() => {}}
                style={{
                  height: 70,
                  borderTopRightRadius: 10,
                  borderTopLeftRadius: 10,
                  marginTop: 30,
                }}
              />
              <AccountListItem title={'Settings'} onPress={() => {}} style={{height: 70}} />
              <AccountListItem
                title={'Permissions'}
                onPress={() => {}}
                style={{height: 70, borderBottomRightRadius: 10, borderBottomLeftRadius: 10}}
              />
            </View>
          </ScrollView>
        </View>
      </SafeAreaView>
    );
  }
}

export default AccountView;
