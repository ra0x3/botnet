import React from 'react';
import {View, SafeAreaView, StatusBar, Text} from 'react-native';
import Ionicons from 'react-native-vector-icons/Ionicons';
import {color} from '../const';

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
      <View
        style={{
          height: '100%',
          width: '100%',
          display: 'flex',
          // justifyContent: 'center',
          alignItems: 'center',
          borderWidth: 1,
          borderColor: 'red',
        }}
      >
        <View
          style={{
            // alignItems: 'center',
            flexDirection: 'column',
            display: 'flex',
            borderWidth: 1,
            borderColor: 'blue',
            width: '100%',
          }}
        >
          <View
            style={{
              width: '100%',
              height: 200,
              borderWidth: 1,
              display: 'flex',
              // justifyContent: 'center',
              // alignItems: 'center',
              position: 'relative',
            }}
          >
            <Text>Image</Text>
            <View
              style={{
                position: 'absolute',
                bottom: 0,
                borderWidth: 1,
                display: 'flex',
                flexDirection: 'row',
                justifyContent: 'space-between',
                width: '100%',
                height: 50,
              }}
            >
              <View
                style={{
                  borderWidth: 1,
                  borderColor: 'red',
                  height: 50,
                  width: 50,
                  marginLeft: 20,
                }}
              >
                <Text>Icon</Text>
              </View>
              <View
                style={{
                  borderWidth: 1,
                  borderColor: 'red',
                  height: 50,
                  width: 50,
                  marginLeft: 10,
                  display: 'flex',
                  justifyContent: 'center',
                  alignItems: 'center',
                  flexDirection: 'column',
                  marginRight: 20,
                }}
              >
                <Text>Category</Text>
                <Text>Name</Text>
              </View>
            </View>
          </View>
          <View
            style={{
              borderWidth: 1,
              borderColor: 'red',
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              width: '100%',
              height: 250,
            }}
          >
            <View
              style={{
                borderWidth: 1,
                borderColor: 'green',
                width: '90%',
                height: '90%',
                display: 'flex',
                borderRadius: 10,
                backgroundColor: color.white,
                flexDirection: 'column',
              }}
            >
              <View
                style={{
                  borderBottomWidth: 1,
                  borderBottomColor: color.light_grey,
                  height: 50,
                  width: '100%',
                  display: 'flex',
                  flexDirection: 'row',
                }}
              >
                <View
                  style={{
                    borderWidth: 1,
                    width: '70%',
                    height: '100%',
                    display: 'flex',
                    flexDirection: 'column',
                  }}
                >
                  <Text>Title</Text>
                  <Text>Subtitle will go here when read</Text>
                </View>
                <View
                  style={{
                    borderWidth: 1,
                    borderColor: 'red',
                    width: 100,
                    height: '100%',
                    display: 'flex',
                    justifyContent: 'center',
                    alignItems: 'center',
                  }}
                >
                  <Ionicons
                    name={'ios-chevron-forward-outline'}
                    size={20}
                    color={color.light_grey}
                  />
                </View>
              </View>
            </View>
          </View>
          <Text style={{marginLeft: 20, fontSize: 22, fontWeight: 'bold'}}>Access History</Text>
          <View
            style={{
              borderWidth: 1,
              borderColor: 'blue',
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              width: '100%',
              height: 250,
            }}
          >
            <View
              style={{
                borderWidth: 1,
                borderColor: 'green',
                width: '90%',
                height: '90%',
                display: 'flex',
                borderRadius: 10,
                backgroundColor: color.white,
                flexDirection: 'column',
              }}
            >
              <Text>History</Text>
            </View>
          </View>
        </View>
      </View>
    );
  }
}

export default FocusAccessRequestView;
