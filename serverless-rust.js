'use strict';

class ServerlessRust {
  constructor(serverless) {
    this.service = serverless.service
    this.hooks = {
      'deploy:deploy': () => this.beforeDeploy(),
    };
  }

  beforeDeploy() {
    const service = this.service;
    console.log('Service: ', service);
    console.log('Provider name: ', service.provider.name);
    console.log('Functions: ', service.functions);
  }
}

module.exports = ServerlessRust;