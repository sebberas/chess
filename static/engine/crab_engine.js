import * as __SNOWPACK_ENV__ from '../../snowpack/env.js';
import.meta.env = __SNOWPACK_ENV__;


let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

let cachegetInt8Memory0 = null;
function getInt8Memory0() {
    if (cachegetInt8Memory0 === null || cachegetInt8Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt8Memory0 = new Int8Array(wasm.memory.buffer);
    }
    return cachegetInt8Memory0;
}

function getArrayI8FromWasm0(ptr, len) {
    return getInt8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
* Returns a list of places a piece can move, when at a specific position
* @param {number} piece
* @param {Pos} pos
* @param {number} color
* @returns {Int8Array}
*/
export function valid_moves(piece, pos, color) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(pos, Pos);
        var ptr0 = pos.ptr;
        pos.ptr = 0;
        wasm.valid_moves(retptr, piece, ptr0, color);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayI8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @returns {GameState}
*/
export function default_board() {
    var ret = wasm.default_board();
    return GameState.__wrap(ret);
}

/**
* @param {GameState} board
* @param {Pos} a
* @param {Pos} b
* @returns {boolean}
*/
export function board_move(board, a, b) {
    _assertClass(board, GameState);
    _assertClass(a, Pos);
    var ptr0 = a.ptr;
    a.ptr = 0;
    _assertClass(b, Pos);
    var ptr1 = b.ptr;
    b.ptr = 0;
    var ret = wasm.board_move(board.ptr, ptr0, ptr1);
    return ret !== 0;
}

/**
* @param {GameState} board
* @param {number} piece
* @param {Pos} pos
* @param {number} color
* @returns {Int8Array}
*/
export function board_valid_moves(board, piece, pos, color) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(board, GameState);
        _assertClass(pos, Pos);
        var ptr0 = pos.ptr;
        pos.ptr = 0;
        wasm.board_valid_moves(retptr, board.ptr, piece, ptr0, color);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayI8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
* @param {GameState} board
* @param {Pos} a
* @param {Pos} b
* @returns {boolean}
*/
export function board_is_valid_move(board, a, b) {
    _assertClass(board, GameState);
    _assertClass(a, Pos);
    var ptr0 = a.ptr;
    a.ptr = 0;
    _assertClass(b, Pos);
    var ptr1 = b.ptr;
    b.ptr = 0;
    var ret = wasm.board_is_valid_move(board.ptr, ptr0, ptr1);
    return ret !== 0;
}

/**
* @param {number} x
* @param {number} y
* @returns {Pos}
*/
export function new_pos(x, y) {
    var ret = wasm.new_pos(x, y);
    return Pos.__wrap(ret);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}
/**
* @param {Board} board
* @param {number} color
* @param {number} depth
* @returns {Int8Array}
*/
export function get_best_move(board, color, depth) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(board, Board);
        var ptr0 = board.ptr;
        board.ptr = 0;
        wasm.get_best_move(retptr, ptr0, color, depth);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var v1 = getArrayI8FromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1);
        return v1;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

/**
*/
export const Piece = Object.freeze({ Pawn:0,"0":"Pawn",Queen:1,"1":"Queen",King:2,"2":"King",Knight:3,"3":"Knight",Bishop:4,"4":"Bishop",Rook:5,"5":"Rook",None:6,"6":"None", });
/**
*/
export const Color = Object.freeze({ White:0,"0":"White",Black:1,"1":"Black", });
/**
*/
export class Board {

    static __wrap(ptr) {
        const obj = Object.create(Board.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_board_free(ptr);
    }
}
/**
*/
export class GameState {

    static __wrap(ptr) {
        const obj = Object.create(GameState.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gamestate_free(ptr);
    }
    /**
    */
    get board() {
        var ret = wasm.__wbg_get_gamestate_board(this.ptr);
        return Board.__wrap(ret);
    }
    /**
    * @param {Board} arg0
    */
    set board(arg0) {
        _assertClass(arg0, Board);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_gamestate_board(this.ptr, ptr0);
    }
    /**
    */
    get winner() {
        var ret = wasm.__wbg_get_gamestate_winner(this.ptr);
        return ret === 2 ? undefined : ret;
    }
    /**
    * @param {number | undefined} arg0
    */
    set winner(arg0) {
        wasm.__wbg_set_gamestate_winner(this.ptr, isLikeNone(arg0) ? 2 : arg0);
    }
}
/**
*/
export class JsPos {

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_jspos_free(ptr);
    }
    /**
    */
    get x() {
        var ret = wasm.__wbg_get_jspos_x(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_jspos_x(this.ptr, arg0);
    }
    /**
    */
    get y() {
        var ret = wasm.__wbg_get_jspos_y(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_jspos_y(this.ptr, arg0);
    }
}
/**
*/
export class Pos {

    static __wrap(ptr) {
        const obj = Object.create(Pos.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_pos_free(ptr);
    }
    /**
    */
    get x() {
        var ret = wasm.__wbg_get_pos_x(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_pos_x(this.ptr, arg0);
    }
    /**
    */
    get y() {
        var ret = wasm.__wbg_get_pos_y(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_pos_y(this.ptr, arg0);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('crab_engine_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

