<script lang="ts">
    export let title: string;
    export let date: string;
    export let published: string;

    export let image: string;
    export let alt: string;

    export let description: string;

    export let link: string;

</script>

<a class="box" href={link}>
    <div class="columns is-vcentered">
        <div class="column is-narrow is-hidden-mobile">
            <figure class="image is-64x64 is-inline-block">
                <img class="is-rounded" src={image} {alt} />
            </figure>
        </div>
        <div class="column">
            <div class="columns mb-0">
                <div class="column">
                    <h4 class="title is-4 mb-0"> {title} </h4>
                </div>
                <div class="column is-hidden-mobile" style="text-align:right">
                    <b class="title is-6 mb-0"> {published} </b>
                    <small style="display: inline"> ({date}) </small>
                </div>
            </div>
            

            <p> {description} </p>
        </div>
    </div>
</a>