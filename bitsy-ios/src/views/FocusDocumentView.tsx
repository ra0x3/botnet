import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import {color} from '../const';

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
            width: '100%',
            display: 'flex',
          }}
        >
          <View
            style={{justifyContent: 'center', alignItems: 'center', display: 'flex', width: '100%'}}
          >
            <View style={{borderWidth: 1, borderColor: 'red', width: '100%'}}>
              <View
                style={{
                  borderWidth: 1,
                  height: 250,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  width: '100%',
                  justifyContent: 'center',
                }}
              >
                <View
                  style={{
                    borderWidth: 1,
                    width: '90%',
                    height: '90%',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                  }}
                >
                  <Text>Image</Text>
                </View>
              </View>

              <View
                style={{
                  borderWidth: 1,
                  height: 250,
                  marginTop: 20,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'row',
                  width: '100%',
                }}
              >
                <View style={{borderWidth: 1, borderColor: 'green', width: '50%', height: 200}}>
                  <View
                    style={{
                      borderWidth: 1,
                      height: 100,
                      width: '100%',
                      display: 'flex',
                      justifyContent: 'center',
                      alignItems: 'center',
                    }}
                  >
                    <View
                      style={{
                        borderWidth: 1,
                        width: '90%',
                        height: '85%',
                        backgroundColor: color.white,
                        borderRadius: 10,
                      }}
                    ></View>
                  </View>
                  <View
                    style={{
                      borderWidth: 1,
                      height: 100,
                      width: '100%',
                      display: 'flex',
                      justifyContent: 'center',
                      alignItems: 'center',
                    }}
                  >
                    <View
                      style={{
                        borderWidth: 1,
                        width: '90%',
                        height: '85%',
                        backgroundColor: color.white,
                        borderRadius: 10,
                      }}
                    ></View>
                  </View>
                </View>
                <View
                  style={{
                    borderWidth: 1,
                    borderColor: 'red',
                    width: '50%',
                    height: 200,
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                  }}
                >
                  <View
                    style={{
                      borderWidth: 1,
                      width: '90%',
                      height: '100%',
                      backgroundColor: color.white,
                      borderRadius: 10,
                    }}
                  ></View>
                </View>
              </View>
              <View
                style={{
                  borderWidth: 1,
                  height: 150,
                  borderColor: 'blue',
                  display: 'flex',
                  flexDirection: 'column',
                  alignItems: 'center',
                  width: '100%',
                  justifyContent: 'center',
                }}
              >
                <View
                  style={{
                    borderWidth: 1,
                    width: '95%',
                    height: '90%',
                    backgroundColor: color.white,
                    borderRadius: 10,
                  }}
                ></View>
              </View>
            </View>
          </View>
        </View>
      </SafeAreaView>
    );
  }
}

export default FocusDocumentView;
