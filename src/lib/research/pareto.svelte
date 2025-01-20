<script lang="ts">
    import { Line } from 'svelte-chartjs';
    import 'chart.js/auto';

    const metrics = {
        'vanilla': {l0: [6.71, 21.4, 51.6, 148.8], add: [0.470, 0.1363, 0.06, 0.029], nmse: [0.3328, 0.1651, 0.1166, 0.079]},
        'vanilla + lookup': {l0: [2.8, 12.9, 38.9, 115], add: [0.420, 0.1645, 0.0667, 0.031], nmse: [0.2934, 0.170, 0.1152, 0.080]},
        'gated': {l0: [3.291, 8.845, 20.512, 56.717], add: [0.537, 0.160, 0.055, 0.024], nmse: [0.3392, 0.1689, 0.1165, 0.0805]},
        'top-k': {l0: [5, 10, 20, 30, 50], add: [0.309, 0.116, 0.047, 0.034, 0.023], nmse: [0.233, 0.154, 0.107, 0.093, 0.077]},
        'top-k + lookup': {l0: [5, 10, 20, 30, 50], add: [0.137, 0.077, 0.036, 0.027, 0.015], nmse: [0.163, 0.126, 0.092, 0.080, 0.065]}
    };

    const colors = ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF'];

    const datasets = (name) => (Object.entries(metrics).map(([key, value], index) => ({
        label: key,
        data: value.l0.map((l0, i) => ({ x: l0, y: value[name][i] })),
        borderColor: colors[index % colors.length],
        backgroundColor: colors[index % colors.length],
        fill: false,
    })));

    const options = (y_axis) => ({
        responsive: true,
        plugins: {
            title: {
                display: false,
            },
            legend: {
                position: 'top',
            }
        },
        scales: {
            x: {
                type: 'logarithmic',
                position: 'bottom',
                title: {
                    display: true,
                    text: 'L0',
                    font: {weight: 'bold'}
                },
            },
            y: {
                type: 'logarithmic',
                position: 'left',
                title: {
                    display: true,
                    text: y_axis,
                    font: {weight: 'bold'}
                }
            }
        }
    });
</script>

<figure>
    <Line data={{datasets: datasets("nmse")}} width={600} height={400} options={options("nmse")}/>
    <Line data={{datasets: datasets("add")}} width={600} height={400} options={options("CE added")}/>
    <figcaption>Pareto curves of the normalized mean squared error (top) and added cross entropy (bottom) across SAE architectures. All SAEs were trained on layer 8 of GPT-2 small with about 300M tokens.</figcaption>
</figure>