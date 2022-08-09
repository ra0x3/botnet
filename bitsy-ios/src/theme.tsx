import {DefaultTheme} from 'react-native-paper';
import {color} from './const';

export default {
  ...DefaultTheme,
  roundness: 2,
  version: 3,
  colors: {
    ...DefaultTheme.colors,
    primary: color.cobalt,
    secondary: 'red',
    tertiary: '#a1b2c3',
  },
};
