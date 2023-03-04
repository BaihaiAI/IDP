declare module '*.png';
declare module '*.jsx';
declare module '*.js';
declare module '*.module.less' {
  const classes: { readonly [key: string]: string };
  export default classes;
}
declare module '*.less';