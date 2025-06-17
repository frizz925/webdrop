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

export interface SessionDto<C extends Content = Content> {
	id: SessionID;
	objects: FileObjectDto<C>[];
}

export interface Session<C extends Content = Content> {
	id: SessionID;
	objects: FileObject<C>[];
}

export const sessionFromDto = (dto: SessionDto) => {
	return {
		id: dto.id,
		objects: dto.objects.map(objectFromDto)
	} as Session;
};

export const objectFromDto = <C extends Content>(dto: FileObjectDto<C>) => {
	return {
		id: dto.id,
		mime: dto.mime,
		content: dto.content,
		timestamp: parseISO(dto.timestamp)
	} as FileObject<C>;
};
