import type { FileContent, FileObject, SessionID } from '$lib/models';

export interface Props {
	sid: SessionID;
	object: FileObject;
	content: FileContent;
}
