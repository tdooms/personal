<script lang="ts">
    import Award from "$lib/award.svelte";
    
    const awards = [
        {
            "title": "3rd place in the Belgian cyber security challenge",
            "date": "(2023)"
        },
        {
            "title": "2nd place on collaborative math olympiad: Wiskunnend Wiske",
            "date": "(2018)"
        },
        {
            "title": "4-time semi-finalist in Flemish science olympiads",
            "date": "(2016 - 2018)"
        },
        {
            "title": "6-time top 100 in Flemish Math Olympiad",
            "date": "(2010 - 2018)"
        },
    ]
</script>


<h5 class="title is-5"> Awards </h5>
<div class="box">
    {#each awards as award}
        <Award {...award} />
    {/each}
</div>