<script lang="ts">
        const programming = [
        {
            "name": "Python",
            "logo": "icons/python.png",
            "time": "6 years",
        },
        {
            "name": "Rust",
            "logo": "icons/rust.jpg",
            "time": "4 years"
        },
        {
            "name": "C++",
            "logo": "icons/cpp.png",
            "time": "3 years",
        },
        {
            "name": "JavaScript",
            "logo": "icons/js.png",
            "time": "1 year"
        },
    ]
</script>

<h5 class="title is-5"> Programming </h5>
<div class="columns is-multiline">
    {#each programming as lang}
        <div class="column is-half">
            <div class="box has-text-centered">
                <figure class="image is-64x64 is-inline-block">
                    <img src={lang.logo} alt={lang.name} style="filter: grayscale(0.5)"/>
                </figure>
                <p class="mt-0"> <b> {lang.time} </b> </p>
            </div>
        </div>
    {/each}
</div>