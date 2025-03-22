import Synthesizer from './synth.js';
const { ipcRenderer } = require('electron');

const synth = new Synthesizer();

document.addEventListener('DOMContentLoaded', () => {
    const startButton = document.getElementById('start');
    const stopButton = document.getElementById('stop');
    const frequencySlider = document.getElementById('frequency');
    const waveformSelect = document.getElementById('waveform');
    const darkModeToggle = document.getElementById('dark-mode-toggle'); // Added
    const spectrogramCanvas = document.getElementById('spectrogram'); // Added

    startButton.addEventListener('click', () => synth.start(frequencySlider.value, waveformSelect.value));
    stopButton.addEventListener('click', () => synth.stop());
    frequencySlider.addEventListener('input', (e) => synth.setFrequency(e.target.value));
    waveformSelect.addEventListener('change', (e) => synth.setWaveform(e.target.value));

    darkModeToggle.addEventListener('click', () => {
        document.body.classList.toggle('dark-mode');
    });

    // Real-time spectrogram rendering
    setInterval(() => {
        const spectrogramData = synth.getSpectrogramData();
        renderSpectrogram(spectrogramCanvas, spectrogramData);
    }, 100);

    ipcRenderer.send('app-ready');

    const recordButton = document.getElementById('record');
    const stopRecordButton = document.getElementById('stop-record');
    const exportButton = document.getElementById('export');

    let recordedBlob = null;

    recordButton.addEventListener('click', () => {
        synth.startRecording();
        console.log('Recording started...');
    });

    stopRecordButton.addEventListener('click', async () => {
        recordedBlob = await synth.stopRecording();
        console.log('Recording stopped.');
    });

    exportButton.addEventListener('click', () => {
        if (recordedBlob) {
            const reader = new FileReader();
            reader.onload = () => {
                ipcRenderer.send('export-audio', reader.result);
            };
            reader.readAsArrayBuffer(recordedBlob);
        } else {
            console.error('No recording available to export.');
        }
    });
});

function renderSpectrogram(canvas, data) {
    const ctx = canvas.getContext('2d');
    const width = canvas.width;
    const height = canvas.height;
    const imageData = ctx.createImageData(width, height);

    for (let i = 0; i < data.length; i++) {
        const value = data[i];
        const color = Math.floor(value * 255);
        imageData.data[i * 4] = color; // Red
        imageData.data[i * 4 + 1] = color; // Green
        imageData.data[i * 4 + 2] = color; // Blue
        imageData.data[i * 4 + 3] = 255; // Alpha
    }

    ctx.putImageData(imageData, 0, 0);
}
