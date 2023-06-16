<script>
    // @ts-nocheck

    import { createEventDispatcher } from "svelte";
    import { sep } from "@tauri-apps/api/path";

    // let fileOpen = false;
    export let name = "file";
    export let path = "";
    // export let dpc = "";
    export let subfiles;
    export let minimized = true;
    export let imported = false;
    $: textColor = getTextColor();
    $: weight = minimized ? "normal" : "bold";
    $: folderIcon = minimized ? "▶" : "▼";
    $: isDir = subfiles != null;

    const dispatcher = createEventDispatcher();
    async function fileAction() {
        if (isDir) minimized = !minimized;
        else if (isDPC()) {
            dispatcher("opendpc", { file: path });
        } else if (isObject()) {
            dispatcher("openobj", { file: path });
        } else if (imported) dispatcher("openimport", { file: path });
        else dispatcher("openfile", { file: path });
    }

    export function isDPC() {
        return path.toLowerCase().endsWith("dpc") && !isDir;
    }

    function isObject() {
        return path.endsWith("_Z") && !isDir;
    }

    function getTextColor() {
        if (isDPC()) return "#f2b3c8";
        else if (isObject()) return "#f2d5a5";
        else return "#f5eee1";
    }
</script>

<div style="--weight: {weight}; --color: {textColor}">
    <button on:click={fileAction}
        >{#if isDir}{folderIcon} {/if}{name}</button
    >
    {#if isDir}
        <div hidden={minimized} class="file-inner">
            {#each [...subfiles.keys()] as file}
                <svelte:self
                    name={file}
                    subfiles={subfiles.get(file)}
                    path={path + sep + file}
                    on:opendpc
                    on:openobj
                    on:openfile
                />
            {/each}
        </div>
    {/if}
</div>

<style>
    * {
        font-family: "DM Sans", sans-serif;
    }

    button {
        overflow: hidden;
        min-width: 100%;
        min-height: 2em;
        text-overflow: clip;
        /* max-height: 2em; */
        text-align: left;
        white-space: nowrap;
        border: 0;
        background-color: hsl(0, 10%, 15%);
        color: var(--color);
        font-weight: var(--weight);
    }

    button:hover {
        background-color: hsl(0, 15%, 20%);
    }

    .file-inner {
        margin-left: 1ch;
        border-left: 3px solid hsl(0, 15%, 25%);
    }
</style>
