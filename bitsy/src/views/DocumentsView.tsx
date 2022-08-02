import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';

interface DocumentsViewState {}

interface DocumentsViewProps {}

class DocumentsView extends React.Component<DocumentsViewProps, DocumentsViewState> {
  constructor(props: DocumentsViewProps) {
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
            <Text style={{marginTop: 20}}>DocumentsView</Text>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default DocumentsView;
