import { parseISO } from 'date-fns';

export interface Content {
	kind: string;
}

export interface TextContent extends Content {
	data: string;
}

export interface LinkContent extends Content {
	url: string;
	title: string;
}

export interface Upload<C> {
	mime: string;
	content: Content & C;
}

export interface FileObjectDto<C extends Content> {
	id: number;
	mime: string;
	timestamp: string;
	content: C;
}

export interface FileObject<C extends Content> {
	id: number;
	mime: string;
	timestamp: Date;
	content: C;
}

export interface SessionDto {
	id: number;
	objects: FileObjectDto<Content>[];
}

export interface Session {
	id: number;
	objects: FileObject<Content>[];
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
