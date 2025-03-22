const { contextBridge } = require('electron');

contextBridge.exposeInMainWorld('api', {
  // Expose APIs here if needed
});
