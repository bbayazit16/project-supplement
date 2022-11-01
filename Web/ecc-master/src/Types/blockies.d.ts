interface IBlockiesOpt {
  seed: string;
  size: number;
  scale: number;
}

declare module '@download/blockies' {
  export const createIcon: (options: IBlockiesOpt) => HTMLCanvasElement;
}
