<script lang="ts">
    let index = 1;
    let model = "noise-strong";

    $: is_index_active = (num: number): string => index == num ? "is-active" : ""
    $: is_model_active = (category, intensity): string => model === `${category.prefix}-${intensity.value}` ? "is-active" : "";

    $: set_index = (i) => index = i;
    $: set_model = (category, intensity) => model = `${category.prefix}-${intensity.value}`;

    $: image = index > 0 ? `/demos/eigenvectors/${model}/pos${index}.svg` : `/demos/eigenvectors/${model}/neg${-index}.svg`;

    const intensities = [
        { value: 'light', label: 'Light' },
        { value: 'medium', label: 'Medium' },
        { value: 'strong', label: 'Strong' },
    ];
    const categories = [
        { label: 'Input Noise', prefix: 'noise', clickable: true },
        { label: 'Translation', prefix: 'translate', clickable: true },
        { label: 'Rotation', prefix: 'rotate', clickable: true }
    ];
    
</script>

<style>
    .is-custom :global() {
    --bulma-tabs-toggle-link-active-background-color: var(--bulma-grey-light);
    --bulma-tabs-toggle-link-active-border-color: var(--bulma-grey-dark);
  }
</style>

<div class="columns">
    <div class="column is-one-fifth">
        <aside class="menu">
            {#each categories as category}
                <p class="menu-label">{category.label}</p>
                <ul class="menu-list">
                    {#each intensities as intensity}
                    <li> <a class={is_model_active(category, intensity)} on:click={() => set_model(category, intensity)}> {intensity.label}</a> </li>
                    {/each}
                </ul>
                {/each}
          </aside>
    </div>
<div class="column">

<div class="columns">
    <div class="column">
        <h4 class="title is-4 has-text-right mt-2"> Positive </h4>
    </div>
    <div class="column">
        <div class="tabs is-toggle is-left is-custom">
            <ul>
                {#each [1, 2, 3, 4, 5] as i}
                <li class={is_index_active(i)}>
                    <a on:click={() => set_index(i)}> {i} </a>
                </li>
                {/each}
            </ul>
        </div>
    </div>
    <!-- <div class="column"></div> -->
    <div class="column">
        <div class="tabs is-toggle is-right is-custom">
            <ul>
                {#each [5, 4, 3, 2, 1] as i}
                <li class={is_index_active(-i)}>
                    <a on:click={() => set_index(-i)}> {i} </a>
                </li>
                {/each}
            </ul>
        </div>
    </div>
    <div class="column">
        <h4 class="title is-4 mt-2"> Negative </h4>
    </div>
</div>

<img src={image} alt="eigenvectors"/>

<div class="notification is-grey-light mb-1 mt-5">
    <p class="block">
        Eigenvectors have a different interpretation that heatmaps. The contribution toward the output is strictly defined by whether it has positive or negative eigenvalue.  
        Strokes of the same color positively interfere while strokes of opposite color negatively interfere.
        Hence, many strokes can be seen as localized edge-detectors.
    </p>

    <p class="block">
        Positive eigenvectors show structure that correspond to important strokes or proto-digits.
        Negative eigenvectors show 'inhibitory' strokes that would aversely affect the classification.
    </p>

</div>

<small class="ml-1"> Any suggestions for other models/settings? <a href="mailto:doomsthomas@gmail.com">Let me know.</a></small>

</div>
</div>