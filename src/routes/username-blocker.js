// username-blocker.js
import { readTextFile, writeTextFile, exists, BaseDirectory } from '@tauri-apps/api/fs';

const PATTERNS_FILE = 'blocked-usernames.txt';

const DEFAULT_PATTERNS = [
    '# gClient Username Filters',
    '# One RegExp expression per line.',
    '# If a username matches at least one expression, the messages will be blocked.',
    '# Lines that are empty or begin with a hashtag (#) will be ignored.',
    '',
    '# Spam bot filters',
    'californiagurls\\d\\d\\d\\d\\d#twoblade\.com'
].join('\n');

let regexPatterns = [];
let isInitialized = false;

export async function initializeUsernameFilters() {
    try {
        const fileExists = await exists(PATTERNS_FILE, { dir: BaseDirectory.AppConfig });
    
        if (fileExists) {
            const content = await readTextFile(PATTERNS_FILE, { dir: BaseDirectory.AppConfig });
            loadUsernameFilters(content);
            console.log('Loaded existing username blocking patterns');
        } else {
            console.log('Creating default username patterns file...');
            await writeTextFile(PATTERNS_FILE, DEFAULT_PATTERNS, { dir: BaseDirectory.AppConfig });
            loadUsernameFilters(DEFAULT_PATTERNS);
            console.log('Created default username blocking patterns');
        }
    
        isInitialized = true;
    } catch (error) {
        console.error('Error initializing username patterns:', error);
        loadUsernameFilters(DEFAULT_PATTERNS);
        isInitialized = true;
    }
}

function loadUsernameFilters(content) {
    regexPatterns = content
        .split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0)
        .filter(line => !line.startsWith('#'));
        
    console.log(`Loaded ${regexPatterns.length} username blocking patterns`);
}

export function testUsername(username) {
    if (!isInitialized) {
        console.warn('Username blocker not initialized yet');
        return false;
    }
    
    if (!username || typeof username !== 'string') {
        return false;
    }
    
    return regexPatterns.some(pattern => {
        try {
            const regex = new RegExp(pattern, 'i');
            return regex.test(username);
        } catch (e) {
            console.warn(`Invalid regex pattern: ${pattern}`, e);
            return false;
        }
    });
}

export async function getAllUsernameFilters() {
    try {
        return await readTextFile(PATTERNS_FILE, { dir: BaseDirectory.AppConfig });
    } catch (error) {
        console.error('Error reading patterns:', error);
        return DEFAULT_PATTERNS;
    }
}

export function getLoadedPatterns() {
  return regexPatterns;
}
