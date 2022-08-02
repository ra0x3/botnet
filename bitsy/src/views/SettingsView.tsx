import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface SettingsViewState {}

interface SettingsViewProps {}

class SettingsView extends React.Component<SettingsViewProps, SettingsViewState> {
  constructor(props: SettingsViewProps) {
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
            <Text style={{marginTop: 20}}>SettingsView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default SettingsView;
