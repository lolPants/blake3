declare module '*/native/index.node' {
  export function blake3(
    buf: Buffer,
    callback: (err: Error, data: Buffer) => any
  ): undefined
}
