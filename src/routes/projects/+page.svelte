<script lang="ts">
    import Project from "$lib/project.svelte";

    let academic = [
    {
      "name": "meme.ir",
      "link": "https://github.com/tdooms/meme.ir",
      "date": "2022",
      "description": "A group project in which we finetuned a transformer model to predict a meme template given only the text. We achieved 70% top-1 accuracy on 100 templates."
    },
    {
      "name": "GNMX",
      "link": "https://github.com/GNMX-UA/GNMX",
      "date": "2021",
      "description": "An interdisciplinary group project about computational biology. We developed a simulation and visualisation to investigate the genetic polymorphism phenomenon."
    },
    {
      "name": "O4 Compiler",
      "link": "https://github.com/tdooms/o4-compiler",
      "date": "2020",
      "description": "A fully fledged C to MIPS compiler using antlr4 and LLVM. There are several optimisation techniques which make the output quite optimised."
    }
  ]
  let personal = [
    {
      "name": "PixelGuesser",
      "link": "https://github.com/tdooms/pixelguesser",
      "date": "2022",
      "description": "PixelGuesser is a party game where players have to guess the contents of a heavily pixelated image which slowly grows more detailed over time. "
    },
    {
      "name": "Paw",
      "link": "https://github.com/tdooms/paw",
      "date": "2021",
      "description": "Easy to use ray marching renderer using JSON for scene description. Many primitives, composables and textures are supported."
    },
    {
      "name": "C++ ini parser",
      "link": "https://github.com/tdooms/ini-parser",
      "date": "2019",
      "description": "A Pythonic ini parser written in c++ using template hacking. The main goals are speed and ease of use. The ini file is read using a performant custom parser."
    }
  ]
</script>


<div class="columns is-centered">
    <div class="column is-6">
        <h5 class="title is-5"> Personal Projects </h5>
        {#each personal as project}
            <Project {...project} />
        {/each}   
    </div>
    <div class="column is-6">
        <h5 class="title is-5"> Academic Projects </h5>
        {#each academic as project}
            <Project {...project} />
        {/each}        
    </div>
</div>

