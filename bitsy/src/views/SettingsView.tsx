import React from 'react';
import {View, SafeAreaView, StatusBar, FlatList, Text} from 'react-native';
import {List} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import {generateFakeItems} from '../utils';
import {NavigationProps} from '../global';
import Account from '../models/Account';
import Setting from '../models/Setting';

interface SettingsViewItemProps {
  item: Setting;
  navigate: any;
}

interface SettingsViewItemState {}

interface FlatListItemProps {
  item: Setting;
}

class SettingsViewItem extends React.Component<SettingsViewItemProps, SettingsViewItemState> {
  constructor(props: SettingsViewItemProps) {
    super(props);
    this.state = {};
  }

  render = () => {
    const {item, navigate} = this.props;
    return (
      <List.Item
        titleEllipsizeMode={'tail'}
        descriptionEllipsizeMode={'tail'}
        style={{
          borderWidth: 1,
          borderColor: color.light_grey,
          width: '100%',
        }}
        title={item.key}
        description={item.description()}
        right={(props) => {
          return (
            <View
              style={{
                borderWidth: 1,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
              }}
            >
              <Ionicons name={'ios-chevron-forward-outline'} size={25} color={color.light_grey} />
            </View>
          );
        }}
        onPress={() => navigate('FocusAccessRequest')}
      />
    );
  };
}

interface SettingsViewState {
  items: Array<Setting>;
}

interface SettingsViewProps extends NavigationProps {}

class SettingsView extends React.Component<SettingsViewProps, SettingsViewState> {
  constructor(props: SettingsViewProps) {
    super(props);
    this.state = {
      items: generateFakeItems(
        new Setting('Bitsy Setting Key', true, new Account('0x123', '0x3333', 123, '', null)),
        20,
      ),
    };

    this.navigate = this.navigate.bind(this);
  }

  navigate = (view: string) => {
    this.props.navigation.navigate(view);
  };

  renderItem = ({item}: FlatListItemProps) => {
    return <SettingsViewItem navigate={this.navigate} item={item} key={String(item.key)} />;
  };

  render = () => {
    return (
      <SafeAreaView>
        <StatusBar />
        <View
          style={{
            height: '100%',
            display: 'flex',
            flexDirection: 'column',
          }}
        >
          <View style={{borderWidth: 1, borderColor: 'red', height: 100, width: '100%'}}></View>
          <View
            style={{
              display: 'flex',
              width: '100%',
              borderWidth: 1,
              borderColor: 'blue',
            }}
          >
            <FlatList
              data={this.state.items}
              renderItem={this.renderItem}
              keyExtractor={(item) => item.key}
            />
          </View>
        </View>
      </SafeAreaView>
    );
  };
}

export default SettingsView;
