import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface FocusDocumentViewState {}

interface FocusDocumentViewProps {}

class FocusDocumentView extends React.Component<FocusDocumentViewProps, FocusDocumentViewState> {
  constructor(props: FocusDocumentViewProps) {
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
            <Text style={{marginTop: 20}}>FocusDocumentView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default FocusDocumentView;
