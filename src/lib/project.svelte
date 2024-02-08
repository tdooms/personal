<script lang="ts">
    export let name: string;
    export let date: string;
    export let description: string;
    export let link: string;
</script>

<a class="box" href={link} target="_blank">
    <div class="media">
        <div class="media-left">
            <span class="icon is-large">
                <i class="fab fa-2x fa-github"></i>
            </span>
        </div>
        <div class="media-content">
            <h4 class="title is-4 mb-0"> {name} </h4>
            <h6> {date} </h6>
        </div>
    </div>
    <p> {description} </p>
</a>