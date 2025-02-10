<script lang="ts">
    export let page: string = '';

    const is_active = (name: string): string => page === name ? "is-active" : "";
    let burger = false;
</script>

<div class="columns is-hidden-mobile">
    <div class="column">
        <div class="tabs is-boxed is-right">
            <ul>
                <li class={is_active("blog")}>
                <a href="/blog">
                    <span class="icon is-small"><i class="fas fa-fire" aria-hidden="true"></i></span>
                    <span>Blog</span>
                </a>
                </li>
                <li class={is_active("research")}>
                <a href="/research">
                    <span class="icon is-small"><i class="fas fa-atom" aria-hidden="true"></i></span>
                    <span>Research</span>
                </a>
                </li>
            </ul>
        </div>
    </div>

    <div class="column is-narrow">
        <a href="/"> <b style="font-weight: 800" class="title is-3"> {"tdooms"} </b> </a>
    </div>

    <div class="column">
        <div class="tabs is-boxed is-left">
            <ul>
                <li class={is_active("projects")}>
                <a href="/projects">
                    <span class="icon is-small"><i class="fas fa-code" aria-hidden="true"></i></span>
                    <span>Projects</span>
                </a>
                </li>
                <li class={is_active("resume")}>
                <a href="/resume">
                    <span class="icon is-small"><i class="far fa-file-alt" aria-hidden="true"></i></span>
                    <span>Resume</span>
                </a>
                </li>
            </ul>
        </div>
    </div>
</div>

<div class="is-hidden-desktop">
    <div class="columns is-mobile">
        <div class="column is-3">
            <a class="navbar-burger {burger ? 'is-active' : ''}" role="button" aria-label="menu" aria-expanded={burger} on:click={() => burger = !burger}>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </a>
        </div>
        <div class="column is-6 has-text-centered">
            <a href="."> <b style="font-weight: 800" class="title is-3"> {"tdooms"} </b> </a>
        </div>
    </div>
    <div class="navbar-menu {burger ? 'is-active' : ''}">
        <div class="navbar-end">
            <a class="navbar-item has-text-centered" href="/blog">
                <span class="icon is-small"><i class="fas fa-fire" aria-hidden="true"></i></span>
                <span>Blog</span>
            </a>
            <a class="navbar-item has-text-centered" href="/research">
                <span class="icon is-small"><i class="fas fa-atom" aria-hidden="true"></i></span>
                <span>Research</span>
            </a>
            <a class="navbar-item has-text-centered" href="/projects">
                <span class="icon is-small"><i class="fas fa-code" aria-hidden="true"></i></span>
                <span>Projects</span>
            </a>
            <a class="navbar-item has-text-centered" href="/resume">
                <span class="icon is-small"><i class="far fa-file-alt" aria-hidden="true"></i></span>
                <span>Resume</span>
            </a>
        </div>  
    </div>
</div>