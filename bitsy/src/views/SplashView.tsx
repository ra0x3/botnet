import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface SplashViewState {}

interface SplashViewProps {}

class SplashView extends React.Component<SplashViewProps, SplashViewState> {
  constructor(props: SplashViewProps) {
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
            <Text style={{marginTop: 20}}>SplashView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default SplashView;
