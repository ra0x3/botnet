import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface PermissionsViewState {}

interface PermissionsViewProps {}

class PermissionsView extends React.Component<PermissionsViewProps, PermissionsViewState> {
  constructor(props: PermissionsViewProps) {
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
            <Text style={{marginTop: 20}}>PermissionsView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default PermissionsView;
