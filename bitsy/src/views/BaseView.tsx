import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface BaseViewState {}

interface BaseViewProps {}

class BaseView extends React.Component<BaseViewProps, BaseViewState> {
  constructor(props: BaseViewProps) {
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
            <Text style={{marginTop: 20}}>BaseView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default BaseView;
