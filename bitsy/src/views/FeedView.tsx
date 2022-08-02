import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface FeedViewState {}

interface FeedViewProps {}

class FeedView extends React.Component<FeedViewProps, FeedViewState> {
  constructor(props: FeedViewProps) {
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
            <Text style={{marginTop: 20}}>FeedView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default FeedView;
