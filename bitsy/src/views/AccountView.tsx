import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface AccountViewState {}

interface AccountViewProps {}

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
            justifyContent: 'center',
            alignItems: 'center',
          }}
        >
          <View style={{justifyContent: 'center', alignItems: 'center', display: 'flex'}}>
            <Text style={{marginTop: 20}}>AccountView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default AccountView;
