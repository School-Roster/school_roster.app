// scripts/download-model.js
import fs from 'fs';
import path from 'path';
// import https from 'https';
import followRedirects from 'follow-redirects';
const { https } = followRedirects;
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';

// Get current directory in ES modules
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Create models directory in src-tauri/
const modelsDir = path.join(__dirname, '../src-tauri/models');
if (!fs.existsSync(modelsDir)) {
  fs.mkdirSync(modelsDir, { recursive: true });
  console.log('Created models directory');
}

// We'll use a tiny ONNX model for text generation
const MODEL_FILES = [
  {
    name: 'tokenizer.json',
    url: 'https://huggingface.co/Xenova/distilgpt2/resolve/main/tokenizer.json'
  },
  {
    name: 'model.onnx',
    url: 'https://huggingface.co/Xenova/distilgpt2/resolve/main/onnx/model_quantized.onnx'
  },
  {
    name: 'config.json',
    url: 'https://huggingface.co/Xenova/distilgpt2/resolve/main/config.json'
  },
  {
    name: 'generation_config.json',
    url: 'https://huggingface.co/Xenova/distilgpt2/resolve/main/generation_config.json'
  }
];

// Download each file
async function downloadFile(url, filePath) {
  return new Promise((resolve, reject) => {
    console.log(`Downloading ${url}...`);
    const file = fs.createWriteStream(filePath);
    
    https.get(url, (response) => {
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }

      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        console.log(`Downloaded ${filePath}`);
        resolve();
      });
    }).on('error', (err) => {
      fs.unlink(filePath, () => {});
      reject(err);
    });
  });
}

// Download all model files
async function downloadModelFiles() {
  try {
    for (const file of MODEL_FILES) {
      const filePath = path.join(modelsDir, file.name);
      await downloadFile(file.url, filePath);
    }
    console.log('All model files downloaded successfully');
  } catch (error) {
    console.error('Error downloading model files:', error);
    process.exit(1);
  }
}

// Run the download
downloadModelFiles();
