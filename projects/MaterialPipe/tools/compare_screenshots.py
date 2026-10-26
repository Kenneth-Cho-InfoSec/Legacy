#!/usr/bin/env python3
"""Perceptual screenshot comparison for the MaterialPipe harness.

Usage: compare_screenshots.py <baseline.png> <current.png> [mean_threshold] [pixel_ratio]

Fails (exit 1) when the current capture visually regresses:
  - dimensions differ, or
  - mean per-channel absolute difference exceeds `mean_threshold`, or
  - more than `pixel_ratio` of pixels differ by > 60 per channel.
Pure-python PNG decoder keeps this dependency-free (no Pillow/PEP 668 issue).
"""
import struct
import sys
import zlib


def png_pixels(path):
    data = open(path, "rb").read()
    i = 8
    idat = b""
    w = h = 0
    while i < len(data):
        ln = struct.unpack(">I", data[i:i + 4])[0]
        typ = data[i + 4:i + 8]
        if typ == b"IHDR":
            w, h = struct.unpack(">II", data[i + 8:i + 16])
        if typ == b"IDAT":
            idat += data[i + 8:i + 8 + ln]
        i += 12 + ln
    raw = zlib.decompress(idat)
    stride = w * 4
    out = []
    prev = bytearray(stride)
    pos = 0
    for _ in range(h):
        f = raw[pos]
        pos += 1
        line = bytearray(raw[pos:pos + stride])
        pos += stride
        if f == 1:
            for x in range(4, stride):
                line[x] = (line[x] + line[x - 4]) & 0xFF
        elif f == 2:
            for x in range(stride):
                line[x] = (line[x] + prev[x]) & 0xFF
        elif f == 3:
            for x in range(stride):
                a = line[x - 4] if x >= 4 else 0
                line[x] = (line[x] + ((a + prev[x]) >> 1)) & 0xFF
        elif f == 4:
            for x in range(stride):
                a = line[x - 4] if x >= 4 else 0
                b = prev[x]
                c = prev[x - 4] if x >= 4 else 0
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                pr = a if (pa <= pb and pa <= pc) else (b if pb <= pc else c)
                line[x] = (line[x] + pr) & 0xFF
        prev = line
        out.append(bytes(line))
    return w, h, out


def main():
    base, current = sys.argv[1], sys.argv[2]
    mean_threshold = float(sys.argv[3]) if len(sys.argv) > 3 else 15.0
    pixel_ratio_max = float(sys.argv[4]) if len(sys.argv) > 4 else 0.40

    w1, h1, r1 = png_pixels(base)
    w2, h2, r2 = png_pixels(current)
    if (w1, h1) != (w2, h2):
        print(f"sizes differ: baseline {w1}x{h1}, current {w2}x{h2}")
        sys.exit(1)

    total = 0.0
    pixels = 0
    diff_pixels = 0
    for y in range(h1):
        a, b = r1[y], r2[y]
        for x in range(w1):
            o = x * 4
            d = abs(a[o] - b[o]) + abs(a[o + 1] - b[o + 1]) + abs(a[o + 2] - b[o + 2])
            total += d / 3.0
            pixels += 1
            if d > 180:
                diff_pixels += 1

    mean = total / pixels
    ratio = diff_pixels / pixels
    print(f"mean diff {mean:.2f}, strong-diff ratio {ratio:.2%}")
    if mean > mean_threshold or ratio > pixel_ratio_max:
        print(f"REGRESSION: mean {mean:.2f} > {mean_threshold} or ratio {ratio:.2%} > {pixel_ratio_max:.0%}")
        sys.exit(1)
    sys.exit(0)


if __name__ == "__main__":
    main()
