import {radar, get_point} from './pkg/wasm.js';

interface Rader {
  print: string;
  getPoint: (scale: number) => string;
}

export const rader = (score: [number], animate = false): Rader => {
  const scale = animate ? 1 : 0;
  const print = radar(new Int32Array(score), scale);

  const getPoint = (scale: number) => {
    return get_point(new Int32Array(score), scale);
  };

  return {
    print,
    getPoint,
  };
};
