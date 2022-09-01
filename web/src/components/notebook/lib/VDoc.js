class VCell {
    constructor(cellId, startLine, textArray) {
        if (textArray.length == 0) {
            textArray.push("");
        }
        this.cellId = cellId;
        this.startLine = startLine;
        this.text = textArray.join("");
        // when init, not endsWith '\n', add it. Editor will not reach this character,
        // so will not change it.
        // if this new cell is added by editor, must send didChange to LSP.
        const lastLine = textArray[textArray.length -1];
        if (! lastLine.endsWith('\n')) {
            this.text += '\n';
        }
        // add seperate new line
        this.endLine = this.startLine + textArray.length - 1;
    }
    containsLine(line){
        return this.startLine <= line && line <= this.endLine;
    }

    rangeSize(){
        return this.endLine - this.startLine + 1;
    }
}

class VDoc {
    /**
     * When didOpen, init VDoc, ensure all cell endsWith '\n' in VDoc.
     * @param uri
     * @param ipynbContentStr
     * @param storeStat
     */
    constructor(uri, ipynbContentStr, storeStat) {
        this.cellMap = {};
        this.currentNewCellBase = 0;
        this.text = "";
        this.version = 0;
        this.uri = uri;
        this.queryId = -1;
        this.storeStat = storeStat;

        const ipynbJson = JSON.parse(ipynbContentStr);
        for (const cell of ipynbJson.cells) {
          if (cell.cell_type === 'code') {
            if ((cell.source instanceof Array && cell.source.join('').trim().startsWith('!'))
              || (typeof cell.source === 'string' && cell.source.trim().startsWith('!'))) continue;
            const sureArray = cell.source instanceof Array ? cell.source : cell.source.split('\n');
            this.addVcell(cell.metadata.id, sureArray);
          }
        }
    }

    addVcell(id, strArray){
        const vCell = new VCell(id, this.currentNewCellBase,
            strArray);
        this.cellMap[id] = vCell;
        this.text += vCell.text;
        this.currentNewCellBase += vCell.rangeSize();
    }

    nextQueryId() {
        this.queryId = this.queryId + 1;
        return this.queryId;
    }

    lastQueryId(){
        return this.queryId;
    }

    /**
     * ensure cell id exist
     * @param cellId
     * @return if exist, return false; else, new cell, return true
     */
    ensureCellId(cellId) {
        if (cellId in this.cellMap) {
            return false;
        } else {
            this.addVcell(cellId, [""]);
            return true;
        }
    }

    cellLineToVline(cellId, cellLine = 0) {
        return this.cellMap[cellId].startLine + cellLine;
    }

    vLineToCellLine(vLine) {
        for (const [cellId, vCell] of Object.entries(this.cellMap)) {
            if (vCell.containsLine(vLine)) {
                return {cellId: cellId, line: vLine - vCell.startLine};
            }
        }
        return {cellId: "unknown", line: -1};
    }


    /**
     * For holding 'cell -> startLine' right, edits which added or deleted '\n's
     * need change startLineMap below this cell.
     * 1，cut multiple lines
     * 2, paste multiple lines
     * 3, edit (change the last line, last character) eat the added fake '\n'
     * 4, last line not end with '\n'
     * @param cellId
     * @param editsInOneCell
     */
    adjustEdits(cellId, editsInOneCell) {
        for (const edit of editsInOneCell) {
            const isNewCell = this.ensureCellId(cellId);
            // change line to vLine
            edit.range.start.line = this.cellLineToVline(cellId, edit.range.start.line);
            edit.range.end.line = this.cellLineToVline(cellId, edit.range.end.line);
            // adjust new lines or deleting lines
            const editStartLine = edit.range.start.line;
            const editLines = edit.range.end.line - edit.range.start.line;
            const textContainLines = (edit.text.match(/\n/g) || []).length;
            const newLineCount = textContainLines - editLines;


            // handle new cell need new line didChange
            if (isNewCell && ! edit.text.endsWith('\n')){
                edit.text += '\n';
            }

            if (newLineCount === 0) { // edit in same line. the most time; this judge is not needed, but only for clear logic;
                continue;
            } else { // diff < 0, delete some lines; diff > 0 add new lines;
                this.currentNewCellBase += newLineCount;
                for (const [cellId, vCell] of Object.entries(this.cellMap)) {
                    // change this cell and all below cell
                    if (vCell.startLine > editStartLine) {
                        vCell.endLine += newLineCount;
                        vCell.startLine += newLineCount;
                    } else if (vCell.containsLine(editStartLine)) {
                        vCell.endLine += newLineCount;
                    }
                }
            }
        }
    }
}


/* for test only

const fs = require('fs');
const ipynb_c = fs.readFileSync('/Users/liuzhe/diagnostics.ipynb', 'utf-8');

const vdoc1 = new VDoc("ttt1.ipynb", ipynb_c);
const cell1 = 'c22069b3-39e8-41e5-b189-95e393d11595';

console.info(vdoc1);
console.info(vdoc1.cellLineToVline(cell1, 15));

console.info(vdoc1.vLineToCellLine(10));



const edit1 = {
    range: {
        start: {
            line: 3,
            charactor: 4
        },
        end: {
            line:3,
            charactor: 4
        },
    },
    text: "helo"
};

const edit2 = JSON.parse(JSON.stringify(edit1));
edit2.text = "hhhh\nkkk\n";

const edit3 = JSON.parse(JSON.stringify(edit1));
edit3.range.end = {line: 11, charactor: 4};

const edits = [edit1, edit2, edit3];

vdoc1.adjustEdits(cell1, edits);
console.log(edit1);
console.log(edit2);
console.log(edit3);
console.log(vdoc1);
 */

export {VDoc}
