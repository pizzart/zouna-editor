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
    .container {
        max-height: 100%;
        /* margin: 4px; */
        border: 3px solid hsl(0, 15%, 20%);
    }

    header {
        background-color: hsl(0, 15%, 20%);
        color: antiquewhite;
    }

    .inner {
        overflow-y: scroll;
        overflow-x: hidden;
        max-height: 92%;
        /* max-height: 70%; */
        display: flex;
        flex-direction: column;
    }

    .game,
    .dpc,
    .imported {
        overflow-y: hidden;
    }

    .game {
        grid-area: left-top;
        /* height: 1fr; */
    }

    .dpc {
        grid-area: left-mid;
    }

    .imported {
        grid-area: left-bot;
    }

    .preview {
        /* overflow: hidden;
        resize: horizontal; */
        grid-area: mid;
        /* width: 100%; */
    }

    .properties {
        grid-area: right;
    }
</style>
