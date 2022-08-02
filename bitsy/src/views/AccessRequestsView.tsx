import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface AccessRequestsViewState {}

interface AccessRequestsViewProps {}

class AccessRequestsView extends React.Component<AccessRequestsViewProps, AccessRequestsViewState> {
  constructor(props: AccessRequestsViewProps) {
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
            <Text style={{marginTop: 20}}>AccessRequestsView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default AccessRequestsView;
