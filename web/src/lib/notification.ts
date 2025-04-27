import { parseISO } from 'date-fns';

export interface NotificationEventDto<T = unknown> {
	name: string;
	data: T;
	timestamp: string;
}

export interface NotificationEvent<T = unknown> {
	name: string;
	data: T;
	timestamp: Date;
}

export type NotificationHandler = (evt: NotificationEvent) => void;

export interface NotificationHandlers {
	[key: string]: NotificationHandler;
}

export const notificationEventFromDto = <T>(dto: NotificationEventDto<T>) => {
	return {
		name: dto.name,
		data: dto.data,
		timestamp: parseISO(dto.timestamp)
	} as NotificationEvent<T>;
};
