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

            <h6 class="subtitle is-6"> Machine Learning Researcher </h6>

            <hr />
            <div class="block">
                <span class="icon"><i class="fas fa-envelope"></i> </span>
                <span> doomsthomas@gmail.com </span>
            </div>

            <div class="block">
                <span class="icon"><i class="fas fa-phone"></i> </span>
                <span> +32478523443 </span>
            </div>

            <div class="block">
                <span class="icon"><i class="fab fa-linkedin"></i> </span>
                <span> <a href="https://www.linkedin.com/in/thomas-dooms-5518a4169/"> LinkedIn </a> </span>
            </div>
            
            <div class="block">
                <span class="icon"><i class="fab fa-github"></i> </span>
                <span> <a href="https://github.com/tdooms"> GitHub </a> </span>
            </div>
        </div>
    </div>
    <div class="column">
        <div class="box">
            <div class="content">
                <p> Hi, I'm Thomas! I'm a first year PhD student working on mechanistic interpretability in machine learning. </p>

                <p> 
                    My research focusses on understanding neural networks from their weights. 
                    This allows us to make guarantees about their behaviour and failure modes.
                </p>

                <p> 
                    Machine learning models have become an undeniably useful tool in recent years. 
                    Yet, they remain shrouded in mystery; both in research and everyday use. 
                    Understanding these models will not only help their trustworthiness but guarantee their utility.
                </p>

                <p>
                    Outside of work, I love to go on long bike rides, game with friends, go skiing and wear shorts.
                </p>
            </div>
        </div>
    </div>
</div>
