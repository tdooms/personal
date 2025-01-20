<script lang="ts">
    import Navbar from "$lib/navbar.svelte";
</script>

<Navbar />

<div class="columns">
    <div class="column is-3">
        <div class="box has-text-centered">
            <figure class="image is-128x128 is-inline-block">
                <img class="is-rounded" src="profile.jpg" alt="me :)" />
            </figure>
            <h4 class="subtitle is-4"> <b> Thomas Dooms </b> </h4>
            <!-- <h6 class="subtitle is-6"> Antwerp, Belgium </h6> -->

            <h6 class="subtitle is-6"> Interpretability Researcher </h6>

            <hr />
            <div class="block">
                <span class="icon"><i class="fas fa-envelope"></i> </span>
                <span> <a href="mailto:doomsthomas@gmail.com"> Email </span>
            </div>

            <div class="block">
                <span class="icon"><i class="fab fa-linkedin"></i> </span>
                <span> <a href="https://www.linkedin.com/in/thomas-dooms-5518a4169/"> LinkedIn </a> </span>
            </div>
            
            <div class="block">
                <span class="icon"><i class="fab fa-github"></i> </span>
                <span> <a href="https://github.com/tdooms"> GitHub </a> </span>
            </div>
            <div class="block">
                <span class="icon"> <i class="fa-brands fa-google-scholar"></i> </span>
                <span> <a href="https://scholar.google.com/citations?user=PPakd2AAAAAJ&hl=en"> Scholar </a> </span>
            </div>
        </div>
    </div>
    <div class="column">
        <div class="box">
            <div class="content">
                <p> Hi, I'm Thomas! I'm a second year PhD student working on mechanistic interpretability. </p>

                <p>
                    My academic interest lies in understanding neural networks from their weights. <br>
                    I believe this to be the most promising avenue to understanding what a model has learned. <br>
                    Tensor decompositions help us understand what is important to the model.
                </p>

                <p>
                    I am guided by curiosity, strive for simplicity, and try to be effective.
                </p>

                <p>
                    Outside of work, I love to go on long bike rides, game with friends, go skiing and wear shorts.
                </p>
            </div>
        </div>
    </div>
</div>
