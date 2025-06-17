import type { FileContent, FileObject, ObjectID, SessionID } from '$lib/models';

export interface Props {
	sid: SessionID;
	object: FileObject;
	content: FileContent;
	onDelete?: (oid: ObjectID) => void;
}
