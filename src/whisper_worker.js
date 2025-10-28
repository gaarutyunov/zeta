// Placeholder for Whisper Web Worker
// In a full implementation, this would load and run Transformers.js Whisper model

export async function transcribe_with_whisper(audioData) {
    // This would use @xenova/transformers to run Whisper locally
    // Example:
    // import { pipeline } from '@xenova/transformers';
    // const transcriber = await pipeline('automatic-speech-recognition', 'Xenova/whisper-tiny.en');
    // const result = await transcriber(audioData);
    // return result.text;

    throw new Error("Local Whisper not yet implemented. Please use Claude, OpenAI, or Google Cloud.");
}

export function is_whisper_loaded() {
    return false;
}
