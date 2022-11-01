interface IRequestOptions {
  method: string;
  params?: any[];
}

declare global {
  interface Window {
    ethereum: {
      request: (options: IRequestOptions) => Promise<string[]> | Promise<string>;
      on: (event: string, handler: (info?: any) => void) => void;
    };
    gun: any;
    sea: any;
  }
}

export default global;
