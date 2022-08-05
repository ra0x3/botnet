import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface FocusAccessRequestViewState {}

interface FocusAccessRequestViewProps {}

class FocusAccessRequestView extends React.Component<
  FocusAccessRequestViewProps,
  FocusAccessRequestViewState
> {
  constructor(props: FocusAccessRequestViewProps) {
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
            <Text style={{marginTop: 20}}>FocusAccessRequestView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default FocusAccessRequestView;
