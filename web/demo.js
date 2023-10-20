class FrameRateCounter extends HTMLElement {
  constructor() {
    super();
    this._samples = [];
    const shadowRoot = this.attachShadow({
      mode: 'closed'
    });
    shadowRoot.innerHTML = `
   		<div><div id="frame_rate"></div><label for="frame_rate">Frame rate</label></div>
    `;
    this._frame_rate_element = shadowRoot.querySelector('#frame_rate');
  }

  stamp(ts) {
    const NUM_SAMPLES = 10;
    if (!this._last_stamp) {
      this._last_stamp = ts;
      return;
    }
    const sample = ts - this._last_stamp;
    this._last_stamp = ts;
    if (this._samples.length < NUM_SAMPLES) {
      this._samples.push(sample);
      if (this._samples.length == NUM_SAMPLES) {
        this._next = 0;
      }
    } else {
      this._samples[this._next] = sample;
      this._next += 1;
      if (this._next >= NUM_SAMPLES) {
      	this._next = 0;
      }
    }
    this.updateDisplay();
  }

  updateDisplay() {
    var avg = 0.0;
    for (var s of this._samples) {
      avg += s;
    }
    avg /= this._samples.length;
    //console.log(avg, this._samples.length);
    this._frame_rate_element.innerText = (1/(avg / 1000)).toFixed(1);
  }
}

class Demo extends HTMLElement {
  constructor() {
    super();
    this._wasm = null;
    const shadowRoot = this.attachShadow({
      mode: 'closed'
    });
    shadowRoot.innerHTML = `
   		<canvas style="aspect-ratio:640/480;width:100%;"></canvas>
    `;
    this._canvas = shadowRoot.querySelector('canvas');
    this._ctx = this._canvas.getContext('2d');
  }

  get frameCallback() {
    return this._frameCallback;
  }

  set frameCallback(f) {
    this._frameCallback = f;
  }

  #getImageData() {
    const { width, height } = this._canvas.getBoundingClientRect();
    const iwidth = Math.trunc(width);
    const iheight = Math.trunc(height);
    if (!this._imageData || iwidth != this._imageData.width || iheight != this._imageData.height) {
      console.log(`new imageData ${iwidth} ${iheight}`);
      this._imageData = this._ctx.getImageData(0, 0, iwidth, iheight)
    }
  }
  
  async start() {
    this.#getImageData();
    if (this.hasAttribute("src") && this.getAttribute("src")) {
      var path = this.getAttribute("src");
      const importObject = {
        env: {output: (ptr) => {
          const str = new Uint8ClampedArray(this._wasm.exports.memory.buffer, ptr, 1024);
          var s = "";
          for (var i = 0; i < str.length; i++) {
            if (str[i] == 0) {
              break;
            }
            s += String.fromCharCode(str[i]);
          }
          console.log(s)
        }
      },
      };
        
      const response = await fetch(path);
      const wasmBuffer = await response.arrayBuffer();
      const wasmObj = await WebAssembly.instantiate(wasmBuffer, importObject);
        this._wasm = wasmObj.instance;
      
      const { width, height } = this._canvas.getBoundingClientRect();
      const xxx = this._wasm.exports.set_dimensions(32, width, height, width, height, width, height);
      if (width != chosen_width || height != chosen_height) {
         consoe.log("oops");       
      } else {
        this._wasm = null;
      }
    }
    if (this._wasm) {
      this._running = true;
      requestAnimationFrame((ts) => this.animate(ts));
    }
  }

  stop() {
    this._running = false;
  }

  animate(timestamp) {
    if (this._ctx) {
      this.#getImageData();
      const data = this._imageData.data;

      console.log(`rendering(${this._imageData.width}, ${this._imageData.height})`);
      const pointer = this._wasm.exports.render(timestamp, this._imageData.width, this._imageData.height);
      const rendered = new Uint8ClampedArray(this._wasm.exports.memory.buffer, pointer, data.length);
      data.set(rendered);
      this._ctx.putImageData(this._imageData, 0, 0);
    }
    this._frameCallback(timestamp);
    if (this._running) {
      requestAnimationFrame((ts) => {
        this.animate(ts);
      });
    }
  }
}

customElements.define('demo-viewer', Demo);
customElements.define('frame-rate-counter', FrameRateCounter);

