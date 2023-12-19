"use strict";

const __cube = document.querySelector(".cube");

class Vec3 {
  static up = new Vec3(0, 1, 0);
  static right = new Vec3(1, 0, 0);

  constructor(x, y, z) {
    this.x = x;
    this.y = y;
    this.z = z;
  }

  length() {
    return Math.sqrt(this.x ** 2 + this.y ** 2 + this.z ** 2);
  }

  normalize() {
    const length = this.length();

    if (length != 0) {
      this.x /= length;
      this.y /= length;
      this.z /= length;
    }
  }

  static sub(v, t) {
    return new Vec3(
      v.x - t.x,
      v.y - t.y,
      v.z - t.z,
    )
  }
}

class Quat {
  constructor(x, y, z, w) {
    this.x = x;
    this.y = y;
    this.z = z;
    this.w = w;
  }

  static fromAngleAxis(angle, axis) {
    axis.normalize();

    const half = angle / 2;

    const sinHalf = Math.sin(half);
    const cosHalf = Math.cos(half);

    const x = axis.x * sinHalf;
    const y = axis.y * sinHalf;
    const z = axis.z * sinHalf;
    const w = cosHalf;

    return new Quat(x, y, z, w);
  }

  static mul(q, r) {
    return new Quat(
      q.w * r.x + q.x * r.w + q.y * r.z - q.z * r.y,
      q.w * r.y - q.x * r.z + q.y * r.w + q.z * r.x,
      q.w * r.z + q.x * r.y - q.y * r.x + q.z * r.w,
      q.w * r.w - q.x * r.x - q.y * r.y - q.z * r.z,
    );
  }

  apply() {
    __cube.style.transform = `rotate3d(${this.x}, ${this.y}, ${this.z}, ${Math.acos(this.w) * 2}rad)`;
  }
}

let friction = 0.01;
let sensitivity = 0.01;
let velocity = 0;

const orientation = {
  __value: new Quat(0, 0, 0, 1),

  set(value) {
    this.__value = value;
    this.__value.apply();
  },

  get() {
    return this.__value;
  },
};

(() => {
  const mouse = {
    down: false,
    lastMove: 0,
    previous: null,
  };

  const handleUp = () => {
    mouse.down = false;
  };

  document.addEventListener("mouseup", handleUp);
  document.addEventListener("touchend", handleUp);

  const handleMove = (event) => {
    if (!mouse.down) return;

    const newMouse = new Vec3(event.clientX, event.clientY, 0);

    if (window.performance.now() - mouse.lastMove > 100) {
      // This is a fresh scroll.
      mouse.previous = newMouse;
    }

    const delta = Vec3.sub(newMouse, mouse.previous);

    mouse.previous = newMouse;
    mouse.lastMove = window.performance.now();

    const axis = new Vec3(-delta.y, delta.x, 0);
    const rotation = Quat.fromAngleAxis(delta.length() * sensitivity, axis);

    orientation.set(Quat.mul(rotation, orientation.get()));
  };

  document.addEventListener("mousemove", handleMove);
  document.addEventListener("touchmove", (event) => {
    const delta = event.changedTouches[0];

    event.clientX = delta.clientX;
    event.clientY = delta.clientY;

    handleMove(event);
  });

  let angularMomentum = new Vec3(0, 0, 0);

  const handleDown = (event) => {
    // Disables link dragging that occurs when spinning.
    event.preventDefault();

    mouse.down = true;

    angularMomentum = new Vec3(0, 0, 0);
  };

  document.addEventListener("mousedown", handleDown);
  document.addEventListener("touchstart", handleDown);

  let lastUpdate = 0;

  const updateFrame = (timestamp) => {
    if (lastUpdate == 0) lastUpdate = timestamp;

    const delta = (timestamp - lastUpdate) / 1000;
    lastUpdate = timestamp;

    const axis = angularMomentum;
    const omega = angularMomentum.length();
    const decay = Math.exp(-delta * friction);

    const effectiveDelta = friction > 0 ? (1 - decay) / friction : delta;

    let theta = effectiveDelta * omega;

    angularMomentum.x *= decay;
    angularMomentum.y *= decay;
    angularMomentum.z *= decay;

    if (friction > 0 && angularMomentum.length() < 0.00001) {
      theta += angularMomentum.length() / friction;

      angularMomentum.x = 0;
      angularMomentum.y = 0;
      angularMomentum.z = 0;
    }

    const rotation = Quat.fromAngleAxis(theta, axis);

    if (!mouse.down) orientation.set(Quat.mul(rotation, orientation.get()));

    requestAnimationFrame(updateFrame);
  };

  updateFrame(0);
})();
