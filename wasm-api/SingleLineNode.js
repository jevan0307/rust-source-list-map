"use strict";

var wasm = require("./build/source_list_map_wasm_api");

class SingleLineNode extends wasm._SingleLineNode {

    constructor(generatedCode, source, originalSource, startingLine) {
        super(0);
        if (generatedCode) {
            startingLine = startingLine || 1;
            if (source) {
                this.ptr = SingleLineNode._new_String_String_String_Number(generatedCode,
                    source,
                    originalSource,
                    startingLine).ptr;
            } else {
                this.ptr = SingleLineNode._new_String_Null_Null_Number(generatedCode,
                    startingLine).ptr;
            }
        }
	}

    clone() {
        var ret = new SingleLineNode();
        ret.prt = wasm._singlelinenode__clone(this.ptr);
        return ret;
    }
}

module.exports = SingleLineNode;