import React, { useState } from 'react';
import { ThemeProvider } from 'styled-components';
import theme from './theme';

function App() {
  const [darkMode, setDarkMode] = useState(false);

  const toggleDarkMode = () => {
    setDarkMode(!darkMode);
  };

  const currentTheme = {
    ...theme,
    colors: {
      ...theme.colors,
      background: darkMode ? '#333333' : theme.colors.background,
      text: darkMode ? '#FFFFFF' : theme.colors.text,
    },
  };

  return (
    <ThemeProvider theme={currentTheme}>
      <div style={{ backgroundColor: currentTheme.colors.background, color: currentTheme.colors.text }}>
        <h1>Welcome to WaveCrafter</h1>
        <p>Craft and visualize sound waves in real-time.</p>
        <button onClick={toggleDarkMode}>
          Toggle {darkMode ? 'Light' : 'Dark'} Mode
        </button>
      </div>
    </ThemeProvider>
  );
}

export default App;
