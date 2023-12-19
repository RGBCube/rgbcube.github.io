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

class Vec2 extends Vec3 {
  constructor(x, y) {
    super(x, y, 0)
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
    __cube.style.transform = `rotate3d(${this.x}, ${this.y}, ${this.z}, ${this.w * 3.1415}rad)`;
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
    lastMove: window.performance.now(),
    previous: new Vec2(0, 0),
  };

  document.addEventListener("mouseleave", () => {
    mouse.down = false;
  });

  document.addEventListener("mouseup", () => {
    mouse.down = false;
  });

  document.addEventListener("mousedown", (event) => {
    // Disables link dragging that occurs when spinning.
    event.preventDefault();
    mouse.down = true;
  });

  document.addEventListener("mousemove", (event) => {
    if (!mouse.down) return;

    const newMouse = new Vec2(event.clientX, event.clientY);

    if (window.performance.now() - mouse.lastMove > 100) {
      // This is a fresh scroll.
      mouse.previous = newMouse;
    }

    const delta = Vec2.sub(newMouse, mouse.previous);

    mouse.previous = newMouse;
    mouse.lastMove = window.performance.now();

    const rotation = Quat.mul(
      Quat.fromAngleAxis(delta.x * sensitivity, Vec3.up),
      Quat.fromAngleAxis(delta.y * sensitivity, Vec3.right),
    );

    orientation.set(Quat.mul(orientation.get(), rotation));
  });
})();
