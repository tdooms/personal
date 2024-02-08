<script lang="ts">
    export let title: string;
    export let description: string;

    export let date: string;

    export let icon: string;

    export let slides: string;
</script>

<div class="box">
    <div class="columns is-vcentered">
        <div class="column is-narrow">
            <span class="icon is-large">
                <i class="fas fa-2x {icon}"></i>
              </span>
        </div>
        <div class="column">
            <h4 class="title is-4 mb-2" style="display: inline"> {title} </h4>
            <small style="display: inline"> ({date}) </small>
            <p> {description} </p>
        </div>
        <div class="column is-narrow">
            <a class="icon is-medium" href={slides} target="_blank">
                <i class="fa-regular fa-lg fa-file-powerpoint"></i>
            </a>
        </div>
    </div>
</div>