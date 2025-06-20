import { Buffer } from 'buffer';
import type { Content, EncryptedContent, FileObject, Upload } from './models';

export interface CryptoConfig {
	masterKey: CryptoKey;
	authKey: string;
	authKeyURL: string;
}

export interface EncodedCipherParams {
	name: string;
	iv: string;
}

export interface EncodedKDFParams {
	name: string;
	hash: string;
	iterations: number;
	salt: string;
}

const encoder = new TextEncoder();

const AUTH_KEY_CONSTRUCT = encoder.encode('わたしはわたしでいたいから');
export const AUTH_KEY_HEADER = 'X-Auth-Key';

const HKDF_INFO = encoder.encode('WebDrop HKDF');
const SUB_KEY_PARAMS: AesKeyGenParams & AesDerivedKeyParams = {
	name: 'AES-GCM',
	length: 256
};

const MASTER_KEY_USAGES: KeyUsage[] = ['wrapKey', 'unwrapKey'];
const SUB_KEY_USAGES: KeyUsage[] = ['encrypt', 'decrypt'];
const DERIVE_USAGES: KeyUsage[] = ['deriveBits', 'deriveKey'];

export const encodeBuffer = (buf: ArrayBuffer | BufferSource | Uint8Array) => Buffer.from(buf as Uint8Array).toString('base64');
export const decodeBuffer = (b64: string) => Buffer.from(b64, 'base64');

export const createCipherParams = () => ({
	name: 'AES-GCM',
	iv: crypto.getRandomValues(new Uint8Array(12))
} as AesGcmParams);
export const encodeCipherParams = (params: AesGcmParams) => Object.assign({}, params, { iv: encodeBuffer(params.iv) }) as EncodedCipherParams;
export const decodeCipherParams = (params: EncodedCipherParams) => Object.assign({}, params, { iv: decodeBuffer(params.iv) }) as AesGcmParams;

export const createKDFParams = (salt?: Uint8Array | ArrayBuffer | BufferSource) => ({
	name: 'PBKDF2',
	salt: salt || crypto.getRandomValues(new Uint8Array(16)),
	iterations: 600000,
	hash: 'SHA-256'
} as Pbkdf2Params);
export const encodeKDFParams = (params: Pbkdf2Params) => Object.assign({}, params, { salt: encodeBuffer(params.salt) }) as EncodedKDFParams;
export const decodeKDFParams = (params: EncodedKDFParams) => Object.assign({}, params, { salt: decodeBuffer(params.salt) }) as Pbkdf2Params;

export const createMasterKey = async (password: string | Uint8Array, kdfParams?: Pbkdf2Params) => {
	if (typeof password === 'string') password = Buffer.from(password, 'base64');
	if (!kdfParams) kdfParams = createKDFParams();

	const subtle = crypto.subtle;
	const keyMaterial = await subtle.importKey('raw', password, 'PBKDF2', false, DERIVE_USAGES);
	const masterKey = await subtle.deriveBits(kdfParams, keyMaterial, 256);

	return { masterKey, kdfParams };
};

export const importMasterKey = (raw: ArrayBuffer) => crypto.subtle.importKey('raw', raw, 'AES-KW', false, MASTER_KEY_USAGES);

export const createAuthKey = async (masterKey: ArrayBuffer) => {
	const subtle = crypto.subtle;
	const keyMaterial = await subtle.importKey('raw', AUTH_KEY_CONSTRUCT, 'PBKDF2', false, DERIVE_USAGES);
	return await subtle.deriveBits(createKDFParams(masterKey), keyMaterial, 256);
};

export const maybeEncryptUpload = async <C extends Content>(upload: Upload<C>) => {
	const config = getCryptoConfig();
	if (!config) return upload;

	const subtle = crypto.subtle;
	const encoder = new TextEncoder();
	const cipher = createCipherParams();

	const payload = encoder.encode(JSON.stringify(upload));
	const subKey = await subtle.generateKey(SUB_KEY_PARAMS, true, SUB_KEY_USAGES);
	const ciphertext = await subtle.encrypt(cipher, subKey, payload);

	const { masterKey } = config;
	const wrappedKey = await subtle.wrapKey('raw', subKey, masterKey, 'AES-KW');

	return {
		mime: 'application/x-ciphertext',
		content: {
			kind: 'ciphertext',
			cipher: encodeCipherParams(cipher),
			ciphertext: encodeBuffer(ciphertext),
			wrappedKey: encodeBuffer(wrappedKey),
		}
	} as Upload<EncryptedContent>;
};

export const maybeDecryptObject = async <C extends Content>(obj: FileObject<EncryptedContent | C>) => {
	const config = getCryptoConfig();
	if (!config || obj.content.kind !== 'ciphertext') return obj as FileObject<C>;

	const subtle = crypto.subtle;
	const { wrappedKey, cipher, ciphertext } = obj.content as EncryptedContent;
	const { masterKey } = config;
	const subKey = await subtle.unwrapKey('raw', decodeBuffer(wrappedKey), masterKey, 'AES-KW', SUB_KEY_PARAMS, false, SUB_KEY_USAGES);
	const payload = await subtle.decrypt(decodeCipherParams(cipher), subKey, decodeBuffer(ciphertext));

	const decoder = new TextDecoder();
	return Object.assign({}, obj, JSON.parse(decoder.decode(payload))) as FileObject<C>;
}

let cryptoConfig: CryptoConfig | undefined;
export const getCryptoConfig = () => cryptoConfig;
export const setCryptoConfig = (newConfig: CryptoConfig) => (cryptoConfig = newConfig);

export const authorizedRequest = (init?: RequestInit) => {
	const config = getCryptoConfig();
	if (!config) return init;
	return Object.assign({}, init, {
		headers: Object.assign({}, init?.headers, {
			[AUTH_KEY_HEADER]: config.authKey
		})
	});
};

export const authorizedURL = (url: URL | string) => {
	const config = getCryptoConfig();
	if (!config) return url;
	if (url instanceof URL) {
		url.searchParams.set('auth', config.authKeyURL);
		return url;
	}
	const query = `auth=${config.authKeyURL}`;
	return url + (url.indexOf('?') >= 0 ? '&' : '?') + query;
};
