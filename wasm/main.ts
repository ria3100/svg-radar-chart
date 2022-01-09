import {to_string, get_point} from './pkg/wasm.js';

interface Radar {
  print: (scale: number) => string;
  getPoint: (scale: number) => string;
}

export const radar = (score: number[]): Radar => {
  const scoreData = new Int32Array(score);

  const print = (scale: number): string => {
    return to_string(scoreData, scale);
  };

  const getPoint = (scale: number) => {
    return get_point(scoreData, scale);
  };

  return {
    print,
    getPoint,
  };
};
