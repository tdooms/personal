<script lang="ts">
    export let page: string = '';

    const is_active = (name: string): string => page === name ? "is-active" : "";
</script>

<div class="columns">
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
        <a href="."> <b style="font-weight: 800" class="title is-3"> {"tdooms"} </b> </a>
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