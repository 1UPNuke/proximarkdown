export default class MarkdownFile {
    name: string;
    path: string;
    content: string;
    modified: {
        nanos_since_epoch: number,
        secs_since_epoch: number
    };
    saved: boolean = true;

    constructor(
        name: string = "newfile.md", 
        path: string = "",
        modified: {
            nanos_since_epoch: number,
            secs_since_epoch: number
        } = {
            nanos_since_epoch: 0,
            secs_since_epoch: 0
        }, 
        content: string = ""
    ) {
        this.name = name;
        this.path = path;
        this.modified = modified;
        this.content = content;
    }
}
