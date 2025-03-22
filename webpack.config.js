module.exports = {
  entry: './src/index.js', // Ensure this path matches your entry file
  output: {
    path: __dirname + '/dist',
    filename: 'main.js',
  },
  mode: 'production',
};
