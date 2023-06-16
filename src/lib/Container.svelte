<script lang="ts">
    export let name: string = "Container";
    export let className: string = "container";
</script>

<div class="container {className}">
    <header>{name}</header>
    <div class="inner">
        <slot />
    </div>
</div>

<style>
    :root {
        --border-color: hsl(0, 15%, 25%);
    }

    .container {
        max-height: 100%;
        max-width: 100%;
        /* margin: 4px; */
        border: 3px solid var(--border-color);
    }

    header {
        padding: 2px;
        background-color: var(--border-color);
        color: antiquewhite;
    }

    :is(.game, .dpc, .imported, .properties) .inner {
        overflow-y: scroll;
        overflow-x: hidden;
        height: calc(100% - 24px);
        /* max-height: 70%; */
        display: flex;
        flex-direction: column;
    }

    *::-webkit-scrollbar {
        background-color: var(--dark-bg);
        width: 10px;
    }
    *::-webkit-scrollbar-thumb {
        background-color: var(--light-bg);
    }
    *::-webkit-scrollbar-thumb:hover {
        background-color: var(--lighter-bg);
    }

    .preview .inner {
        height: calc(100% - 24px);
    }

    .game,
    .dpc,
    .imported,
    .properties {
        overflow-y: hidden;
    }

    .game {
        grid-area: game;
    }

    .dpc {
        grid-area: dpc;
    }

    .imported {
        grid-area: imported;
    }

    .preview {
        grid-area: preview;
        overflow: hidden;
    }

    .properties {
        grid-area: properties;
    }

    :is(.properties, .preview) .inner {
        padding: 4px;
    }
</style>
