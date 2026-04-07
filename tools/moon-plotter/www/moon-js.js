/**
 * Pure JavaScript sphere renderer.
 * Mirrors the algorithms in sphere/src/sphere.rs and sphere/src/checker.rs.
 */

const FRAC_PI_2 = Math.PI / 2;

/** Wraps x into [0, width). Mirrors Rust's rem_euclid. */
function wrapWidth(x, width) {
    return ((x % width) + width) % width;
}

/**
 * Renders a sphere with an orthographic projection.
 *
 * @param {number} size - Output canvas pixel size (square).
 * @param {number} long0 - Longitude offset in radians (rotates the globe).
 * @param {{ data: Uint8ClampedArray, width: number, height: number }} source - RGBA pixel source.
 * @returns {Uint8ClampedArray} RGBA pixel buffer of size*size*4 bytes.
 */
export function renderSphere(size, long0, source) {
    const { data: srcData, width: srcW, height: srcH } = source;
    const buf = new Uint8ClampedArray(size * size * 4); // defaults to all zeros (black, transparent)

    const globeR = size / 2;
    const srcMidX = srcW / 2;
    const srcMidY = srcH / 2;
    const srcR = srcMidY;

    for (let y = 0; y <= size; y++) {
        // latitude: mirrors latitude(y, globe_r) in sphere.rs
        const sinPhi = (y - globeR) / globeR;
        const lat = Math.asin(Math.max(-1, Math.min(1, sinPhi)));

        const srcY = Math.trunc(srcMidY + (lat / FRAC_PI_2) * srcR);
        if (srcY < 0 || srcY >= srcH || y >= size) continue;

        const smallR = globeR * Math.cos(lat);
        if (smallR <= 0) continue;

        for (let x = 0; x < size; x++) {
            const xd = x - globeR;
            // mirrors the bounds check in longitude() in sphere.rs
            if (xd < -smallR || xd > smallR) continue;

            const lon = Math.asin(xd / smallR);
            const absLon = lon + long0;
            const srcX = Math.trunc(srcMidX + (absLon / FRAC_PI_2) * srcR);
            const srcXWrap = wrapWidth(srcX, srcW);

            const si = (srcY * srcW + srcXWrap) * 4;
            const di = (y * size + x) * 4;
            buf[di]     = srcData[si];
            buf[di + 1] = srcData[si + 1];
            buf[di + 2] = srcData[si + 2];
            buf[di + 3] = 255;
        }
    }

    return buf;
}

/**
 * Generates a checker pattern. Mirrors generate_checker() in sphere/src/checker.rs.
 *
 * @returns {{ data: Uint8ClampedArray, width: number, height: number }}
 */
export function generateChecker(width, height) {
    const data = new Uint8ClampedArray(width * height * 4);
    const checkerSize = 25;
    const halfW = Math.floor(width / 2);
    const halfH = Math.floor(height / 2);

    for (let x = 0; x < width; x++) {
        for (let y = 0; y < height; y++) {
            const a = Math.floor(x / checkerSize) % 2 === 0;
            const b = Math.floor(y / checkerSize) % 2 === 0;
            if (a !== b) { // xor
                const c = Math.floor(x * 256 / width);
                const idx = (y * width + x) * 4;
                if ((x < halfW) !== (y < halfH)) { // xor(x < half_width, y < half_height)
                    data[idx] = c; data[idx + 1] = 0;   data[idx + 2] = 0;
                } else {
                    data[idx] = 0; data[idx + 1] = c;   data[idx + 2] = 0;
                }
                data[idx + 3] = 255;
            }
        }
    }

    return { data, width, height };
}

/**
 * Loads an image URL and returns its RGBA pixel data by drawing it to an offscreen canvas.
 *
 * @param {string} url
 * @returns {Promise<{ data: Uint8ClampedArray, width: number, height: number }>}
 */
export function loadImage(url) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.onload = () => {
            const offscreen = document.createElement("canvas");
            offscreen.width = img.naturalWidth;
            offscreen.height = img.naturalHeight;
            const ctx = offscreen.getContext("2d");
            ctx.drawImage(img, 0, 0);
            const imageData = ctx.getImageData(0, 0, img.naturalWidth, img.naturalHeight);
            resolve({ data: imageData.data, width: img.naturalWidth, height: img.naturalHeight });
        };
        img.onerror = () => reject(new Error(`Failed to load image: ${url}`));
        img.src = url;
    });
}
