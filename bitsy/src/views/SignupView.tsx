import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import {Button} from 'react-native-paper';
import {NavigationProps} from '../global';

interface SignupViewState {}

interface SignupViewProps extends NavigationProps {}

class SignupView extends React.Component<SignupViewProps, SignupViewState> {
  constructor(props: SignupViewProps) {
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
            <Text style={{marginTop: 20}}>SignupView</Text>
            <Button onPress={() => this.props.navigation.replace('Login')}>Login</Button>
            <Button onPress={() => this.props.navigation.replace('Tabs')}>Signup</Button>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default SignupView;
