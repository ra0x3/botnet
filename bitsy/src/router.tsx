import React from 'react';
import 'react-native-gesture-handler';
import AccessRequestsView from './views/AccessRequestsView';
import FeedView from './views/FeedView';
import AccountView from './views/AccountView';
import SignupView from './views/SignupView';
import LoginView from './views/LoginView';
import DocumentsView from './views/DocumentsView';
import FocusDocumentView from './views/FocusDocumentView';
import FocusAccessRequestView from './views/FocusAccessRequestView';
import SettingsView from './views/SettingsView';
import Ionicons from 'react-native-vector-icons/Ionicons';
import {NavigationContainer} from '@react-navigation/native';
import {color} from './const';

import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {createStackNavigator} from '@react-navigation/stack';

const Tab = createBottomTabNavigator();
const Stack = createStackNavigator();

const StackNavigator = () => {
  return (
    <NavigationContainer>
      <Stack.Navigator
        initialRouteName={'Login'}
        screenOptions={{
          headerShown: false,
        }}
      >
        <Stack.Screen name="Login" component={LoginView} />
        <Stack.Screen name="Signup" component={SignupView} />
        <Stack.Screen name="Settings" component={SettingsView} />
        <Stack.Screen name="FocusedDocument" component={FocusDocumentView} />
        <Stack.Screen name="FocusAccessRequest" component={FocusAccessRequestView} />
        <Stack.Screen name="Tabs" component={TabNavigator} />
      </Stack.Navigator>
    </NavigationContainer>
  );
};

const TabNavigator = () => {
  return (
    <Tab.Navigator
      initialRouteName={'Feed'}
      screenOptions={({route}) => ({
        tabBarIcon: ({focused, color: dcolor, size}) => {
          let iconname: string = '';

          switch (route.name) {
            case 'Feed':
              iconname = 'ios-grid';
              break;
            case 'AccessRequests':
              iconname = 'ios-chatbubble';
              break;
            case 'Documents':
              iconname = 'ios-document';
              break;
            case 'Account':
              iconname = 'ios-person-circle';
              break;
            default:
              iconname = 'ios-airplane';
          }

          iconname = focused ? iconname : `${iconname}-outline`;

          return <Ionicons name={iconname} size={size} color={focused ? color.cobalt : dcolor} />;
        },
        tabBarActiveTintColor: color.cobalt,
        tabBarInactiveTintColor: 'gray',
        tabBarShowLabel: false,
        headerShown: false,
      })}
    >
      <Tab.Screen name="Feed" component={FeedView} />
      <Tab.Screen name="AccessRequests" component={AccessRequestsView} />
      <Tab.Screen name="Documents" component={DocumentsView} />
      <Tab.Screen name="Account" component={AccountView} />
    </Tab.Navigator>
  );
};

export default StackNavigator;
