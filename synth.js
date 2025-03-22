class Synthesizer {
    constructor() {
        this.audioContext = new (window.AudioContext || window.webkitAudioContext)();
        this.oscillator = null;
        this.gainNode = this.audioContext.createGain();
        this.gainNode.gain.value = 0.5; // Default volume
        this.gainNode.connect(this.audioContext.destination);
    }

    start(frequency = 440, type = 'sine') {
        if (this.oscillator) {
            this.stop();
        }
        this.oscillator = this.audioContext.createOscillator();
        this.oscillator.type = type;
        this.oscillator.frequency.setValueAtTime(frequency, this.audioContext.currentTime);
        this.oscillator.connect(this.gainNode);
        this.oscillator.start();
    }

    stop() {
        if (this.oscillator) {
            this.oscillator.stop();
            this.oscillator.disconnect();
            this.oscillator = null;
        }
    }

    setFrequency(frequency) {
        if (this.oscillator) {
            this.oscillator.frequency.setValueAtTime(frequency, this.audioContext.currentTime);
        }
    }

    setWaveform(type) {
        if (this.oscillator) {
            this.oscillator.type = type;
        }
    }

    startRecording() {
        this.mediaRecorder = new MediaRecorder(this.audioContext.destination.stream);
        this.recordedChunks = [];
        this.mediaRecorder.ondataavailable = (event) => {
            if (event.data.size > 0) {
                this.recordedChunks.push(event.data);
            }
        };
        this.mediaRecorder.start();
    }

    stopRecording() {
        return new Promise((resolve) => {
            this.mediaRecorder.onstop = () => {
                const blob = new Blob(this.recordedChunks, { type: 'audio/wav' });
                resolve(blob);
            };
            this.mediaRecorder.stop();
        });
    }

    getSpectrogramData() {
        const bufferLength = 1024;
        const dataArray = new Float32Array(bufferLength);
        this.audioContext.createAnalyser().getFloatFrequencyData(dataArray);
        return dataArray;
    }
}

export default Synthesizer;
