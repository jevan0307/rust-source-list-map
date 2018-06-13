"use strict";

var wasm = require("./build/source_list_map_wasm_api");
var CodeNode = require("./CodeNode");
var SourceNode = require("./SourceNode");
var SingleLineNode = require("./SingleLineNode");
var StringVec = require("./utils").StringVec;
var NodeVec = require("./utils").NodeVec;

class SourceListMap extends wasm._SourceListMap {
    constructor(generatedCode, source, originalSource) {
        super(0);
        if (generatedCode !== -1) {
            if (Array.isArray(generatedCode)) {
                var nodes = NodeVec(generatedCode);
                this.ptr = SourceListMap._new_nodes(nodes);
            } else {
                this.ptr = SourceListMap._new().ptr;
                if (generatedCode || source) {
                    this.add(generatedCode, source, originalSource);
                }
            }
        }
    }

    add(generatedCode, source, originalSource) {
        var nodes = NodeVec([generatedCode]);
        if (source) {
            this._add_node_string_string(nodes, source, originalSource);
        } else {
            this._add_node(nodes);
        }
    }

    prepend(generatedCode, source, originalSource) {
        var nodes = NodeVec([generatedCode]);
        if (source) {
            this._prepend_node_string_string(nodes, source, originalSource);
        } else {
            this._prepend_node(nodes);
        }
    }

    mapGeneratedCode(fnName) {
        var newSlp = new SourceListMap(-1);
        newSlp.ptr = wasm._sourcelistmap_map_generated_code(this, fnName).ptr;
        return newSlp;
    }

    toString() {
        return this._to_string();
    }

    toStringWithSourceMap(options) {
        var srcMap = this._to_string_with_source_map();
        var ret = {
            source: srcMap.get_source(),
            map: {
                file: options.file,
                version: 3,
                mappings: srcMap.get_mappings()
            }
        };

        var sourcesLen = srcMap.get_map_sources_len();
        if (sourcesLen > 0) {
            ret.map.sources = [];
            for (var i = 0; i < sourcesLen; i++) {
                ret.map.sources.push(srcMap.get_map_sources_nth(i));
            }
        } else {
            ret.map.sources = [null];
        }

        var contentsLen = srcMap.get_map_contents_len();
        if (contentsLen > 0) {
            ret.map.sourcesContent = [];
            for (var i = 0; i < contentsLen; i++) {
                ret.map.sourcesContent.push(srcMap.get_map_contents_nth(i));
            }
        }

        srcMap.free();
        return ret;
    }

    toStringWithSourceMapJson(options) {
        var srcMap = JSON.parse(this._to_string_with_source_map_json());
        var ret = {
            source: srcMap.source,
            map: {
                file: options.file,
                version: 3,
                mappings: srcMap.mappings
            }
        };

        ret.map.sources = srcMap.mapSources;
        if (ret.map.sources.length < 0) {
            ret.map.sources = [null];
        }

        ret.map.sourcesContent = srcMap.mapSourcesContent;
        return ret;
    }
}

module.exports = SourceListMap;
