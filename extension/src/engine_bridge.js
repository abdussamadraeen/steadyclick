class CentennialEngine {
  constructor() {
    this.instance = null;
    this.memory = null;
    this.filterPtr = null;
    this.isLoaded = false;
  }

  async init(wasmUrl) {
    const response = await fetch(wasmUrl);
    const bytes = await response.arrayBuffer();
    const { instance } = await WebAssembly.instantiate(bytes, { env: {} });

    this.instance = instance;
    this.memory = instance.exports.memory;
    this.filterPtr = this.instance.exports.one_euro_new(1.0, 0.005, 1.0);
    this.isLoaded = true;
  }

  filterCursor(x, y, timestampSec) {
    if (!this.isLoaded || !this.filterPtr) return { x, y };

    const exports = this.instance.exports;
    const outPtr = exports.centennial_alloc(16);

    exports.one_euro_step(this.filterPtr, x, y, timestampSec, outPtr, outPtr + 8);

    const view = new Float64Array(this.memory.buffer, outPtr, 2);
    const fx = view[0];
    const fy = view[1];

    exports.centennial_dealloc(outPtr, 16);
    return { x: fx, y: fy };
  }

  queryVoronoi(cursorX, cursorY, targets, maxMagneticRadius = 60.0) {
    if (!this.isLoaded || targets.length === 0) {
      return {
        capturedId: null,
        distPrimary: Infinity,
        distSecondary: Infinity,
        snapX: cursorX,
        snapY: cursorY,
      };
    }

    const exports = this.instance.exports;
    const targetCount = targets.length;
    const floatCount = targetCount * 5;
    const byteSize = floatCount * 8;

    const targetsPtr = exports.centennial_alloc(byteSize);
    const targetsView = new Float64Array(this.memory.buffer, targetsPtr, floatCount);

    for (let i = 0; i < targetCount; i++) {
      const offset = i * 5;
      const t = targets[i];
      targetsView[offset] = t.id;
      targetsView[offset + 1] = t.x0;
      targetsView[offset + 2] = t.y0;
      targetsView[offset + 3] = t.x1;
      targetsView[offset + 4] = t.y1;
    }

    const outPtr = exports.centennial_alloc(40);

    exports.voronoi_query(
      cursorX,
      cursorY,
      targetsPtr,
      targetCount,
      maxMagneticRadius,
      outPtr
    );

    const outView = new Float64Array(this.memory.buffer, outPtr, 5);
    const capturedId = outView[0] >= 0 ? Math.round(outView[0]) : null;
    const distPrimary = outView[1];
    const distSecondary = outView[2];
    const snapX = outView[3];
    const snapY = outView[4];

    exports.centennial_dealloc(targetsPtr, byteSize);
    exports.centennial_dealloc(outPtr, 40);

    return { capturedId, distPrimary, distSecondary, snapX, snapY };
  }

  evaluateAPCA(txtR, txtG, txtB, bgR, bgG, bgB) {
    if (!this.isLoaded) return 0;
    return this.instance.exports.apca_evaluate(txtR, txtG, txtB, bgR, bgG, bgB);
  }

  isFamiliarWord(word) {
    if (!this.isLoaded) return true;
    const encoder = new TextEncoder();
    const bytes = encoder.encode(word.toLowerCase());
    const byteSize = bytes.length;

    const ptr = this.instance.exports.centennial_alloc(byteSize);
    const memView = new Uint8Array(this.memory.buffer, ptr, byteSize);
    memView.set(bytes);

    const result = this.instance.exports.dale_chall_lookup(ptr, byteSize);
    this.instance.exports.centennial_dealloc(ptr, byteSize);

    return result === 1;
  }

  destroy() {
    if (this.filterPtr && this.instance) {
      this.instance.exports.one_euro_free(this.filterPtr);
      this.filterPtr = null;
    }
  }
}

window.CentennialEngine = CentennialEngine;
