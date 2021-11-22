declare module "*.svg" {
  export const Component: import("preact").FunctionComponent<
    import("preact").JSX.SVGAttributes
  >;
}

declare module "*.png" {
  const ref: string;
  export default ref;
}

declare module "*.jpg" {
  const ref: string;
  export default ref;
}
