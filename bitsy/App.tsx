import React from 'react';
import {Provider} from 'react-native-paper';
import Navigator from './src/router';

const App = () => {
  return (
    <Provider>
      <Navigator />
    </Provider>
  );
};

export default App;
