<script>
    // @ts-nocheck

    import Container from "$lib/Container.svelte";
    import { save, open } from "@tauri-apps/api/dialog";
    import { exists, createDir } from "@tauri-apps/api/fs";
    // import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";

    let workingDir = "";
    let gameFiles = new Map();
    let relativeGameFiles = [];
    let dpcFiles = new Map();
    let relativeDPCFiles = [];

    let projectDropdown = true;
    let exportDropdown = true;
    function toggleProject() {
        projectDropdown = !projectDropdown;
        exportDropdown = true;
    }
    function toggleExport() {
        exportDropdown = !exportDropdown;
        projectDropdown = true;
    }

    async function saveProject() {
        const filePath = await save({
            filters: [
                {
                    name: "Zouna Edit Project",
                    extensions: ["zep"],
                },
            ],
        });
        console.log(filePath);
    }

    async function openGame() {
        clearGameFiles();
        clearDPCFiles();
        exportDropdown = true;
        projectDropdown = true;
        const dir = await open({
            directory: true,
        });

        workingDir = dir;
        // console.log(working_dir);
        let files = await invoke("read_dir", { dir });
        files.forEach((file) => {
            if (!file.includes("ZOUNA_PROJECT"))
                gameFiles.set(file.replace(dir, ""), file);
        });
        relativeGameFiles = [...gameFiles.keys()];
        // console.log([...game_files.values()]);
    }

    async function openDPC(file) {
        clearDPCFiles();

        const inPath = gameFiles.get(file);
        const outPath = workingDir + "/ZOUNA_PROJECT";

        let dirExists = await exists(outPath);
        if (!dirExists) await createDir(outPath);
        let dir = "";
        try {
            dir = await invoke("read_dpc", { inPath, outPath });
            let files = await invoke("read_dir", { dir });
            files.forEach((file) => {
                dpcFiles.set(file.replace(dir, ""), file);
            });
            relativeDPCFiles = [...dpcFiles.keys()];
        } catch (err) {
            console.log(`error ${err.message}`);
        }
    }

    function clearGameFiles() {
        gameFiles.clear();
        relativeGameFiles = [];
    }

    function clearDPCFiles() {
        dpcFiles.clear();
        relativeDPCFiles = [];
    }
</script>

<div class="window">
    <div class="menubar">
        <button on:click={toggleProject}>project</button>
        <button on:click={toggleExport}>export</button>
        <div class="dropdown" hidden={projectDropdown}>
            <div class="dropdown-menu">
                <button on:click={openGame}>new</button>
                <button>open</button>
                <button on:click={saveProject}>save</button>
                <button>save as</button>
            </div>
        </div>
        <div class="dropdown" hidden={exportDropdown}>
            <div class="dropdown-menu">
                <button>export</button>
            </div>
        </div>
    </div>
    <div class="editor">
        <Container name="Game Structure" className="game">
            {#each relativeGameFiles as file}
                <button
                    class="filebtn"
                    on:click={() => {
                        openDPC(file);
                    }}>{file}</button
                >
            {/each}
        </Container>
        <Container name="DPC Structure" className="dpc">
            {#each relativeDPCFiles as file}
                <button class="filebtn">{file}</button>
            {/each}
        </Container>
        <Container name="Imported Files" className="imported" />
        <Container name="Preview" className="preview" />
        <Container name="Properties" className="properties" />
    </div>
</div>

<style>
    @import url("https://fonts.googleapis.com/css2?family=Inter&display=swap");

    :global(html) {
        height: 100%;
        user-select: none;
        font-family: "Inter", sans-serif;
        color-scheme: dark;
    }

    :global(body) {
        margin: 0;
        height: 100%;
        max-width: 100vw;
        max-height: 100vh;
    }

    .window {
        display: flex;
        background-color: hsl(0, 7%, 9%);
        height: 100%;
        flex-direction: column;
        /* grid-template-rows: min-content 100%; */
        overflow: hidden;
    }

    .dropdown {
        position: absolute;
        /* transform: translateX(10px); */
    }

    .dropdown-menu {
        display: flex;
        border: 4px solid hsl(0, 15%, 20%);
        flex-direction: column;
        min-width: 10ch;
    }

    .dropdown button {
        text-align: left;
    }

    .editor {
        display: grid;
        margin: 4px;
        gap: 4px;
        height: 100%;
        max-height: calc(100% - 30px);
        grid-template-columns: 1fr 2.2fr 1.1fr;
        grid-template-rows: 1fr 1fr 1fr;
        grid-template-areas:
            "left-top mid right"
            "left-mid mid right"
            "left-bot mid right";
    }

    .menubar {
        width: 100%;
        padding-inline: 4px;
        background-color: hsl(0, 10%, 15%);
        /* border-bottom: 2px solid cornflowerblue; */
    }

    .menubar button {
        font-size: 15px;
    }

    .filebtn {
        overflow: hidden;
        max-width: 100%;
        min-height: 2em;
        text-overflow: clip;
        /* max-height: 2em; */
        text-align: left;
        white-space: nowrap;
    }

    button {
        border: 0;
        background-color: hsl(0, 10%, 15%);
        color: antiquewhite;
    }

    button:hover {
        background-color: hsl(0, 15%, 20%);
    }

    /*
    .files div {
        border-bottom: 2px solid black;
    } */

    /* .game,
    .dpc {
        overflow: hidden;
        resize: vertical;
    } */
</style>
