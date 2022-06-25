'use strict';

class ServerlessRust {
  constructor(serverless, options) {
    this.service = serverless.service
    this.options = options;
    this.hooks = {
      'deploy:deploy': () => this.beforeDeploy(),
    };
  }

  beforeDeploy() {
    const service = this.serverless.service;
    console.log('Provider name: ', service.provider.name);
    console.log('Functions: ', service.functions);
  }
}

module.exports = ServerlessRust;