import { promisify } from 'util'
import addon from '../native/index.node'

const blake3 = promisify(addon.blake3)

type Encoding =
  | 'ascii'
  | 'utf8'
  | 'utf-8'
  | 'hex'
  | 'base64'
  | 'utf16le'
  | 'latin1'
  | 'binary'

async function hash(
  data: string | Buffer,
  encoding?: undefined
): Promise<Buffer>
async function hash(data: string | Buffer, encoding: Encoding): Promise<string>
async function hash(
  data: string | Buffer,
  encoding?: Encoding
): Promise<string | Buffer> {
  const buf: Buffer | undefined =
    typeof data === 'string'
      ? Buffer.from(data)
      : Buffer.isBuffer(data)
      ? data
      : undefined

  if (buf === undefined) {
    throw new TypeError(`data must be a string or a Buffer`)
  }

  const hashed = await blake3(buf)
  return encoding !== undefined ? hashed.toString(encoding) : hashed
}

export { hash }
