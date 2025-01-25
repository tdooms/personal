<script lang="ts">
    export let title: string;
    export let date: string;
    export let published: string;

    export let image: string;
    export let alt: string;

    export let description: string;

    export let link: string;
    export let external: boolean = false;

</script>

<style>
    .box {
        transition: transform 0.3s ease-in-out;
    }
    .box:hover {
        transform: scale(1.02);
    }
</style>

<a class="box" href={link} target="{external ? '_blank' : '_self'}">
    <div class="columns mb-0">
        <div class="column">
            <h5 class="title is-5 mb-0"> {title} </h5>
        </div>
        <div class="column is-narrow is-hidden-mobile" style="text-align:right">
            <b class="title is-6 mb-0"> {published} </b>
            <small style="display: inline"> ({date}) </small>
        </div>
    </div>
    

    <p> {description} </p>
</a>