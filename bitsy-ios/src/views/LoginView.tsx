import React from 'react';
import {View, SafeAreaView, StatusBar, Text, Image, TouchableOpacity} from 'react-native';
import {Button, TextInput, Switch, ActivityIndicator} from 'react-native-paper';
import {httpRequest} from '../utils';
import {ActionState, ErrorMessage, NavigationProps, Option} from './../global';
import {color} from '../const';
import Account from '../models/Account';
import Session from '../services/Session';

interface LoginViewState {
  password: string;
  unlockWithFaceId: boolean;
  actionState: ActionState;
  error: Option<ErrorMessage>;
}

interface LoginViewProps extends NavigationProps {}

class LoginView extends React.Component<LoginViewProps, LoginViewState> {
  constructor(props: LoginViewProps) {
    super(props);
    this.state = {
      password: '',
      error: null,
      unlockWithFaceId: false,
      actionState: ActionState.none,
    };
  }

  submitLogin = async () => {
    this.setState({actionState: ActionState.pending});
    const {data, error} = await httpRequest({
      url: '/account/login2',
      method: 'POST',
      data: {
        password: this.state.password,
      },
    });

    if (error) {
      this.setState({actionState: ActionState.error, error});
      return;
    }

    this.setState({actionState: ActionState.success}, () => {
      Session.save(Account.fromObject(data), () => {
        this.props.navigation.navigate('Tabs', {account: Account.fromObject(data)});
      });
    });
  };

  renderLowerAction = () => {
    if (this.state.actionState === ActionState.pending) {
      return <ActivityIndicator animating={true} color={color.black} />;
    }

    if (this.state.actionState === ActionState.error) {
    }

    return (
      <Button
        mode="outlined"
        style={{
          borderRadius: 10,
          width: '90%',
          height: 40,
        }}
        onPress={async () => await this.submitLogin()}
      >
        UNLOCK
      </Button>
    );
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
                height: 200,
                borderWidth: 1,
                borderColor: 'red',
                width: '100%',
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
              }}
            >
              <Text>Image</Text>
            </View>
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
              <Text style={{fontSize: 22, fontWeight: 'bold'}}>Welcome to bitsy</Text>
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
                label="Password"
                mode="outlined"
                value={this.state.password}
                onChangeText={(text) => this.setState({password: text})}
                style={{width: '100%', height: 40}}
              />
              <Image
                source={{
                  uri: 'https://i.imgur.com/lc73Pt5.png',
                }}
                style={{height: 30, width: 30, borderWidth: 1, position: 'absolute', right: 10}}
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
              {this.renderLowerAction()}
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
                Don't have an account?{' '}
                <TouchableOpacity onPress={() => this.props.navigation.navigate('Signup')}>
                  <Text
                    style={{
                      color: 'blue',
                      textDecorationStyle: 'solid',
                      textDecorationLine: 'underline',
                    }}
                  >
                    Signup
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

export default LoginView;
