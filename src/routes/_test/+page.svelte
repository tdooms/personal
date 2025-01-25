<script lang="ts">
    import Navbar from "$lib/navbar.svelte";
    import { onMount } from 'svelte';
    import npyjs from "npyjs";
    import ndarray from "ndarray";

    const images = [{epoch:9, digit: 0, vec: 9, canvas: null}, {epoch:9, digit: 1, vec: 9, canvas: null}, {epoch:9, digit: 2, vec: 9, canvas: null}];

    onMount(() => {
        start();
    });

    const colormap = (value) => {
        const colors = [
            [103, 0, 31],    // -1.0
            [178, 24, 43],   // -0.75
            [214, 96, 77],   // -0.5
            [244, 165, 130], // -0.25
            [253, 219, 199], // -0.125
            [247, 247, 247], // 0
            [209, 229, 240], // 0.125
            [146, 197, 222], // 0.25
            [67, 147, 195],  // 0.5
            [33, 102, 172],  // 0.75
            [5, 48, 97]      // 1.0
        ];

        value = Math.max(-1, Math.min(1, value));

        // Calculate the position and interpolation weight
        const position = (value + 1) * 5;
        const index = Math.floor(position);
        const weight = position - index;

        // Interpolate between two colors
        const c1 = colors[Math.min(index, 10)];
        const c2 = colors[Math.min(index + 1, 10)];

        if(!c1 || !c2) return [0, 0, 0];
        return c1.map((c, i) => Math.round(c * (1 - weight) + c2[i] * weight));
    };

    async function start() {
        let n = new npyjs();
        const x = await n.load("tensors/vecs.npy");
        const array = ndarray(x.data, x.shape);
        
        images.forEach(el => {
            drawImage(array.pick(el.epoch, el.digit, el.vec), el.canvas);
        });
    }

    function drawImage(image, canvas) {
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const imgData = ctx.createImageData(28, 28);

        for (let i = 0; i < 784; i++) {
            const color = colormap(image.get(i)*3 / 255);

            imgData.data[i * 4] = color[0];     // R
            imgData.data[i * 4 + 1] = color[1]; // G
            imgData.data[i * 4 + 2] = color[2]; // B
            imgData.data[i * 4 + 3] = 255;   // A
        }

        ctx.imageSmoothingEnabled = false; // Keep pixelated look when scaling
        ctx.putImageData(imgData, 0, 0);
        ctx.drawImage(canvas, 0, 0, 28, 28, 0, 0, 280, 280);
    }
</script>

<Navbar />

<div class="columns">
    {#each images as element}
        <div class="column is-3">
            <canvas bind:this={element.canvas} width=280 height=280></canvas>
        </div>
    {/each}
</div>
