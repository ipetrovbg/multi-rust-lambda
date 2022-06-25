'use strict';

const path = require('path');
const fs = require('fs');

class ServerlessRust {
  constructor(serverless) {
    this.serverless = serverless
    this.service = serverless.service
    this.hooks = {
      'deploy:deploy': () => this.beforeDeploy(),
      'before:package:package': () => this.beforePackage(),
    };
  }

  beforeDeploy() {
    const service = this.service;
    console.log('Functions: ', service.functions);
  }

  beforePackage() {
    const directoryPath = path.join(__dirname, this.serverless.serviceDir);
//passsing directoryPath and callback function
    fs.readdir(directoryPath, function (err, files) {
      //handling error
      if (err) {
        return console.log('Unable to scan directory: ' + err);
      }
      //listing all files using forEach
      files.forEach(function (file) {
        // Do whatever you want to do with the file
        console.log(file);
      });
    });
  }
}

module.exports = ServerlessRust;