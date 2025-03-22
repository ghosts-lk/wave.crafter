import React from 'react';
import { ThemeProvider } from 'styled-components';
import theme from './theme';

function App() {
  return (
    <ThemeProvider theme={theme}>
      <div style={{ backgroundColor: theme.colors.background, color: theme.colors.text }}>
        <h1>Welcome to WaveCrafter</h1>
        <p>Craft and visualize sound waves in real-time.</p>
      </div>
    </ThemeProvider>
  );
}

export default App;
