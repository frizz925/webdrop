import type { Content, FileContent, FileObject, ObjectID } from '$lib/models';

export interface Props {
	object: FileObject;
	content: FileContent;
	getFileUrl: (object: FileObject, content: Content) => string;
	onDelete?: (oid: ObjectID) => void;
}
