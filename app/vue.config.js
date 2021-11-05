const fs = require('fs')

module.exports = {
  devServer: {
    https: {
      key: fs.readFileSync('./127.0.0.1-key.pem'),
      cert: fs.readFileSync('./127.0.0.1.pem'),
    },
    public: 'https://127.0.0.1:8080/'
  },
};
