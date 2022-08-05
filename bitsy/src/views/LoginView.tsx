import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import {Button} from 'react-native-paper';
import {NavigationProps} from './../global';

interface LoginViewState {}

interface LoginViewProps extends NavigationProps {}

class LoginView extends React.Component<LoginViewProps, LoginViewState> {
  constructor(props: LoginViewProps) {
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
            <Text style={{marginTop: 20}}>LoginView</Text>
            <Button onPress={() => this.props.navigation.replace('Signup')}>Signup</Button>
            <Button onPress={() => this.props.navigation.replace('Tabs')}>Login</Button>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default LoginView;
