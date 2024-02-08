<script lang="ts">
    export let title: string;
    export let description: string;

    export let date: string;

    export let image: string;
    export let alt: string;

    export let paper: string;
    export let code: string;

</script>

<div class="box">
    <div class="columns is-vcentered">
        <div class="column is-narrow">
            <figure class="image is-64x64 is-inline-block">
                <img class="is-rounded" src={image} {alt} />
            </figure>
        </div>
        <div class="column">
            <h4 class="title is-4 mb-2" style="display: inline"> {title} </h4>
            <small style="display: inline"> ({date}) </small>
            <p> {description} </p>
        </div>
        <div class="column is-narrow">
            <a class="icon is-medium" href={paper} target="_blank">
                <i class="fa-regular fa-lg fa-file"></i>
            </a>
            <hr class="my-3" />
            <a class="icon is-medium" href={code} target="_blank">
                <i class="fab fa-lg fa-github"></i>
            </a>
        </div>
    </div>
</div>