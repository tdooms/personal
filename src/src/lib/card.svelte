<script lang="ts">
    export let title: string;
    export let date: string;
    export let image: string;
    export let location: string;
</script>

<style>
    .card .card-image img {
        filter: grayscale(0.75);
        transition: filter 0.3s ease-in-out;
    }

    .card:hover .card-image img {
        filter: grayscale(0);
    }

    .card {
        transition: transform 0.3s ease-in-out;
    }
    .card:hover {
        transform: scale(1.02);
    }
</style>

<div class="card">
    <div class="card-image">
        <figure class="image is-3by2">
        <img src={image} alt="Placeholder" style="object-fit: cover;">
        </figure>
    </div>
    <div class="card-content">
        <h4 class="title is-4 mb-2"> {title} </h4>
        <div class="content"> {date} </div>
    </div>
    <div class="card-footer">
        <a href={location} class="button is-primary is-fullwidth" style="border-top-left-radius: 0 !important;border-top-right-radius: 0 !important">Read</a>
    </div>
</div>