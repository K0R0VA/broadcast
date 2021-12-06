const fs = require('fs')

module.exports = {
  devServer: {
    https: {
      key: fs.readFileSync('./192.168.0.7-key.pem'),
      cert: fs.readFileSync('./192.168.0.7.pem'),
    },
    public: 'https://192.168.0.7:8080/'
  },
};
