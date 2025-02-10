<script lang="ts">
    export let name: string;
    export let level: string;
    export let percent: number;
</script>

<div class="columns">
    <div class="column is-narrow has-text-left py-0">
        <strong> {name} </strong>
    </div>
    <div class="column has-text-right py-0">
        {level}
    </div>
</div>
<progress class="progress is-danger" style="height:0.5rem" max=100 value={percent}> {percent} % </progress>