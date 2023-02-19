import React from 'react';
import {View} from 'react-native';
import {List, Appbar} from 'react-native-paper';
import {color} from '../const';
import Ionicons from 'react-native-vector-icons/Ionicons';
import AccessRequest, {AccessRequestStatus} from './../models/AccessRequest';
interface AccessRequestsViewItemProps {
  item: AccessRequest;
  navigate: any;
}

interface AccessRequestsViewItemState {}

export interface FlatListItemProps {
  item: AccessRequest;
}

class AccessRequestsViewItem extends React.Component<
  AccessRequestsViewItemProps,
  AccessRequestsViewItemState
> {
  constructor(props: AccessRequestsViewItemProps) {
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
        title={item.formattedTitle()}
        description={item.formattedDescription()}
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

export default AccessRequestsViewItem;
