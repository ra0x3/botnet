import React from 'react';
import {Provider as PaperProvider} from 'react-native-paper';
import Navigator from './src/router';
import theme from './src/theme';

// https://github.com/callstack/react-native-paper/blob/main/src/styles/themes/v3/LightTheme.tsx

const App = () => {
  return (
    <PaperProvider theme={theme}>
      <Navigator />
    </PaperProvider>
  );
};

export default App;
