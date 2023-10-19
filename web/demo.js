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
    this._frame_rate_element.innerText = Math.round(1/(avg / 1000));
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
   		<canvas></canvas>
    `;
    this._canvas = shadowRoot.querySelector('canvas');

  }

  get wasm() {
    return this._value;
  }

  set wasm(v) {
    this._wasm = v;
  }

  start() {
    this._running = true;
    requestAnimationFrame((ts) => this.animate(ts));
  }

  stop() {
    this._running = false;

  }

  animate(timestamp) {
    const canvas = this._canvas;
    const ctx = canvas.getContext('2d');

    if (ctx) {
      const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
      const data = imageData.data;

      for (let i = 0; i < data.length; i += 4) {
        const noise = Math.random() * 255;
        data[i] = noise; // Red
        data[i + 1] = noise; // Green
        data[i + 2] = noise; // Blue
        data[i + 3] = 255; // Alpha (fully opaque)
      }

      ctx.putImageData(imageData, 0, 0);
    }
    if (this._running) {
      requestAnimationFrame((ts) => {
        this.animate(ts);
      });
    }
  }
}

customElements.define('demo-viewer', Demo);
customElements.define('frame-rate-counter', FrameRateCounter);

