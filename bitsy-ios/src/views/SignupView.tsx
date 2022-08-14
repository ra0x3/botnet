import React from 'react';
import {View, SafeAreaView, StatusBar, Text, Image, TouchableOpacity} from 'react-native';
import {Button, TextInput, Switch} from 'react-native-paper';
import {httpRequest} from '../utils';
import {NavigationProps, ActionState, ErrorMessage, Option} from './../global';
import Session from '../services/Session';

interface SignupViewState {
  mnemonic: string;
  password: string;
  confirmedPassword: string;
  unlockWithFaceId: boolean;
  actionState: ActionState;
  error: Option<ErrorMessage>;
}

interface SignupViewProps extends NavigationProps {}

class SignupView extends React.Component<SignupViewProps, SignupViewState> {
  constructor(props: SignupViewProps) {
    super(props);
    this.state = {
      mnemonic: '',
      password: '',
      confirmedPassword: '',
      unlockWithFaceId: false,
      error: null,
      actionState: ActionState.none,
    };
  }

  validateForm = () => {
    if (this.state.password === this.state.confirmedPassword) {
      return true;
    }

    this.setState({actionState: ActionState.error, error: 'Invalid form'});
  };

  importMnemonic = async () => {
    this.setState({actionState: ActionState.pending});

    const {data, error} = await httpRequest({
      url: '/account/mnemonic',
      method: 'POST',
      data: {
        mnemonic: this.state.mnemonic,
        password: this.state.password,
      },
    });

    if (error) {
      this.setState({actionState: ActionState.error, error});
      return;
    }

    this.setState({actionState: ActionState.success}, () => {
      Session.save(data, () => {
        this.props.navigation.navigate('Tabs', {account: data});
      });
    });
  };

  renderErrorHeader = () => {
    if (this.state.actionState === ActionState.error) {
      // TODO: finish
    }
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
            justifyContent: 'center',
            alignItems: 'center',
            padding: 20,
          }}
        >
          <View
            style={{
              alignItems: 'center',
              display: 'flex',
              borderWidth: 1,
              width: '100%',
              height: '100%',
              flexDirection: 'column',
            }}
          >
            <View
              style={{
                height: 70,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
              }}
            >
              <Text style={{fontSize: 22, fontWeight: 'bold'}}>Import new wallet</Text>
            </View>
            <View
              style={{
                height: 100,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                position: 'relative',
              }}
            >
              <TextInput
                label="Mnemonic"
                mode="outlined"
                multiline={true}
                value={this.state.mnemonic}
                onChangeText={(text) => this.setState({mnemonic: text})}
                style={{width: '100%', height: 100}}
              />
            </View>
            <View
              style={{
                height: 75,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                position: 'relative',
              }}
            >
              <TextInput
                label="New Password"
                mode="outlined"
                // secureTextEntry={true}
                value={this.state.password}
                onChangeText={(text) => this.setState({password: text})}
                style={{width: '100%', height: 40}}
              />
            </View>

            <View
              style={{
                height: 75,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                position: 'relative',
              }}
            >
              <TextInput
                label="Confirm Password"
                mode="outlined"
                value={this.state.confirmedPassword}
                onChangeText={(text) => this.setState({confirmedPassword: text})}
                style={{width: '100%', height: 40}}
              />
            </View>
            <View
              style={{
                height: 75,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
              }}
            >
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
                <View
                  style={{
                    borderWidth: 1,
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    width: 150,
                    height: 75,
                  }}
                >
                  <Text>Unlock with Face ID?</Text>
                </View>
                <View
                  style={{
                    borderWidth: 1,
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                    width: 150,
                    height: 75,
                  }}
                >
                  <Switch
                    value={this.state.unlockWithFaceId}
                    onValueChange={() =>
                      this.setState({unlockWithFaceId: !this.state.unlockWithFaceId})
                    }
                  />
                </View>
              </View>
            </View>

            <View
              style={{
                height: 75,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
              }}
            >
              <Button
                mode="outlined"
                style={{borderRadius: 10, width: '90%', height: 40}}
                onPress={async () => await this.importMnemonic()}
              >
                IMPORT
              </Button>
            </View>
            <View
              style={{
                height: 75,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
              }}
            >
              <Text style={{textAlign: 'center'}}>
                Forgot your password? You can recover your wallet using your seed phrase here.
              </Text>
              <Text style={{marginTop: 10}}>
                Already have an account?{' '}
                <TouchableOpacity onPress={() => this.props.navigation.navigate('Login')}>
                  <Text
                    style={{
                      color: 'blue',
                      textDecorationStyle: 'solid',
                      textDecorationLine: 'underline',
                    }}
                  >
                    Signin
                  </Text>
                </TouchableOpacity>
              </Text>
            </View>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default SignupView;