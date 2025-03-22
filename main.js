const { app, BrowserWindow, ipcMain, dialog } = require('electron');
const path = require('path');
const fs = require('fs');

let mainWindow;

app.on('ready', () => {
    mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            preload: path.join(__dirname, 'preload.js'),
            contextIsolation: true,
            enableRemoteModule: false,
        },
    });

    mainWindow.loadFile('index.html');
});

ipcMain.on('app-ready', () => {
    console.log('Synthesizer app is ready.');
});

ipcMain.on('export-audio', (event, audioBuffer) => {
    const savePath = dialog.showSaveDialogSync({
        title: 'Save Audio File',
        defaultPath: 'synth_audio.wav',
        filters: [{ name: 'Audio Files', extensions: ['wav'] }],
    });

    if (savePath) {
        fs.writeFileSync(savePath, Buffer.from(audioBuffer));
        console.log(`Audio saved to ${savePath}`);
    }
});

app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit();
    }
});
