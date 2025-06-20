import { parseISO } from 'date-fns';

export type ObjectID = number | string;
export type SessionID = number | string;

export interface Content {
	kind: string;
}

export interface TextContent extends Content {
	data: string;
	isSecret?: boolean;
}

export interface LinkContent extends Content {
	url: string;
	title?: string;
}

export interface FileContent extends Content {
	name: string;
}

export interface EncryptedContent extends Content {
	cipher: ContentCipherParams;
	ciphertext: string;
	wrappedKey: string;
}

export interface ContentCipherParams {
	name: string;
	iv: string;
}

export interface Upload<C extends Content = Content> {
	mime: string;
	content: C;
}

export interface FileObjectDto<C extends Content = Content> {
	id: ObjectID;
	mime: string;
	timestamp: string;
	content: C;
}

export interface FileObject<C extends Content = Content> {
	id: ObjectID;
	mime: string;
	timestamp: Date;
	content: C;
}

export interface Session {
	id: SessionID;
	crypto?: SessionCrypto;
}

export interface SessionCrypto {
	kdfParams: SessionKDFParams;
}

export interface SessionKDFParams {
	name: string;
	hash: string;
	salt: string;
	iterations: number;
}

export interface SessionAuthParams {
	name: string;
	iv: string;
}

export const objectFromDto = <C extends Content>(dto: FileObjectDto<C>) => {
	return {
		id: dto.id,
		mime: dto.mime,
		content: dto.content,
		timestamp: parseISO(dto.timestamp)
	} as FileObject<C>;
};
