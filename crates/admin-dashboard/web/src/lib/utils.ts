const KILOBYTE = 1024;
const MEGABYTE = KILOBYTE * 1024;
const GIGABYTE = MEGABYTE * 1024;
const TERABYTE = GIGABYTE * 1024;

/**
 * Converts number of bytes to human-readable string representing the size,
 * e.g.:
 * - 100 -> "100 bytes"
 * - 2123 -> "2.1KB"
 * - 79238742 -> "75.6 MB"
 */
export const humanreadableSize = (bytes: number) => {
  const round = (num: number) => Math.round(num * 10) / 10;
  const digits = bytes.toString().length;
  if (digits <= 3) {
    return `${round(bytes)} bytes`;
  } else if (digits <= 6) {
    return `${round(bytes / KILOBYTE)}KB`;
  } else if (digits <= 9) {
    return `${round(bytes / MEGABYTE)}MB`;
  } else if (digits <= 12) {
    return `${round(bytes / GIGABYTE)}GB`;
  } else {
    return `${round(bytes / TERABYTE)}TB`;
  }
};
