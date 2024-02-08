<script lang="ts">
    export let icon: string;
    export let title: string;
    export let date: string;
</script>

<div class="box">
    <div class="media">
        <div class="media-left">
            <figure class="image is-48x48">
                <img src={icon} alt="icon" />
            </figure>
        </div>
        <div class="media-content">
            <h5 class="title is-5 mb-0"> {title} </h5>
            <h6> {date} </h6>
        </div>
    </div>
    <slot />
</div>