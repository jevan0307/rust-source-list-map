"use strict";

var wasm = require("./build/source_list_map_wasm_api");

class CodeNode extends wasm._CodeNode {

    constructor(generatedCode) {
        super(0);
        if (generatedCode) {
            this.ptr = CodeNode._new_String(generatedCode).ptr;
        }
	}

    clone() {
        var ret = new CodeNode();
        ret.prt = wasm._codenode__clone(this.ptr);
        return ret;
    }
}

module.exports = CodeNode;
