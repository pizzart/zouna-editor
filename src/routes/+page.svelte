<script>
    // @ts-nocheck

    import Container from "$lib/Container.svelte";
    import { save, open } from "@tauri-apps/api/dialog";
    import {
        createDir,
        readDir,
        exists,
        readTextFile,
        copyFile,
        // removeDir,
    } from "@tauri-apps/api/fs";
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import { basename, extname, join, sep } from "@tauri-apps/api/path";
    import { listen } from "@tauri-apps/api/event";
    import { panzoom } from "svelte-pan-zoom";
    import File from "../lib/File.svelte";
    // import { Canvas, T, useLoader } from "@threlte/core";
    // import { OrbitControls } from "@threlte/extras";
    import * as THREE from "three";
    // import { GLTFLoader } from "three/addons/loaders/GLTFLoader.js";
    import { OrbitControls } from "three/addons/controls/OrbitControls.js";
    import { OBJLoader } from "three/addons/loaders/OBJLoader.js";
    // import Property from "../lib/Property.svelte";
    import { isRegistered, register } from "@tauri-apps/api/globalShortcut";
    import { onMount } from "svelte";

    const PROJECT_DIR = "ZOUNA_PROJECT";
    const FORMATS = new Map([
        ["audio", ["wav"]],
        ["image", ["png", "gif", "jpg", "jpeg"]],
        ["model", ["glb", "gltf", "obj"]],
    ]);

    let game = "walle";
    $: isProjectOpen = workingDir == "";
    let workingDir = "";
    let currentDPC = "";
    let gameFiles = new Map();
    let dpcFiles = new Map();
    let importFiles = new Map();
    let openObjects = new Map();
    let currentObject;
    $: objectPreview = currentObject
        ? openObjects.get(currentObject).get("preview")
        : null;
    $: objectProperties = currentObject
        ? openObjects.get(currentObject).get("properties")
        : null;
    $: objectReplacement = currentObject
        ? openObjects.get(currentObject).get("replacement")
        : null;
    // let previewScene;
    let sceneContainer;
    $: {
        if (sceneContainer && objectPreview) {
            sceneContainer.innerHTML = "";
            initScene();
            animate();
        }
    }
    let camera;
    let scene;
    let renderer;
    // let model;

    function initScene() {
        renderer = new THREE.WebGLRenderer();
        renderer.setSize(
            sceneContainer.offsetWidth,
            sceneContainer.offsetHeight
        );
        scene = new THREE.Scene();
        camera = new THREE.PerspectiveCamera(
            75,
            sceneContainer.offsetWidth / sceneContainer.offsetHeight,
            0.1,
            1000
        );
        const loader = new OBJLoader();
        const controls = new OrbitControls(camera, renderer.domElement);
        sceneContainer.appendChild(renderer.domElement);
        // const cubeGeo = new THREE.BoxGeometry(1, 1, 1);
        // const cubeMat = new THREE.MeshBasicMaterial({ color: 0xffffff });
        // let model = new THREE.Mesh(cubeGeo, cubeMat);
        // scene.add(model);
        loader.load(
            objectPreview,
            (newModel) => {
                scene.add(newModel);
            },
            undefined,
            function (error) {
                console.error(error);
            }
        );

        const ambientLight = new THREE.AmbientLight(0x404040);
        scene.add(ambientLight);
        const light = new THREE.PointLight(0xffffee, 1, 0, 0.5);
        light.position.set(5, 5, 2);
        scene.add(light);

        camera.position.z = 5;
        controls.update();
    }

    function animate() {
        requestAnimationFrame(animate);
        // renderer.clear();
        renderer.render(scene, camera);
    }

    function resizeScene() {
        renderer.setSize(
            sceneContainer.offsetWidth,
            sceneContainer.offsetHeight
        );
        camera.aspect =
            sceneContainer.offsetWidth / sceneContainer.offsetHeight;
        camera.updateProjectionMatrix();
    }

    onMount(async () => {
        await register("Control+N", openGame);
        await register("Control+O", openGame);
    });

    let dropdowns = new Map([
        ["project", false],
        ["export", false],
        ["game", false],
    ]);

    function toggleDropdown(dropdown) {
        const state = !dropdowns.get(dropdown);
        for (const dd of dropdowns.keys()) {
            dropdowns.set(dd, false);
        }
        dropdowns.set(dropdown, state);
        dropdowns = dropdowns;
    }

    const sortByExtension = ([a], [b]) => {
        const extComp = String(a.split(".")[1]).localeCompare(b.split(".")[1]);
        if (extComp == 0) return String(a).localeCompare(b);
        else return extComp;
    };

    const sortByName = ([a], [b]) => {
        String(a).localeCompare(b);
    };

    function processEntries(entries, sorting) {
        let new_dict = new Map();
        if (entries != null) {
            for (const entry of entries) {
                if (entry.name != "ZOUNA_PROJECT")
                    new_dict.set(
                        entry.name,
                        processEntries(entry.children, sorting)
                    );
            }
            return new Map([...new_dict].sort(sorting));
        } else {
            return null;
        }
    }

    listen("tauri://file-drop", async (event) => {
        if (workingDir) importNewFiles(event.payload);
    });

    // listen("tauri://close-requested", () => {
    //     if (workingDir != "") {
    //         const zpDir = workingDir + sep + PROJECT_DIR;
    //         removeDir(zpDir, { recursive: true });
    //     }
    // });

    async function readImportFiles() {
        const dir = await join(workingDir, PROJECT_DIR, "IMPORTS");
        const files = await readDir(dir, { recursive: true });
        importFiles = processEntries(files, sortByName);
    }

    async function importNewFiles(filesToImport) {
        const dir = await join(workingDir, PROJECT_DIR, "IMPORTS");
        let dirExists = await invoke("path_exists", { path: dir });
        if (!dirExists) await createDir(dir, { recursive: true });
        for (const filePath of filesToImport) {
            const fileName = await basename(filePath);
            const copiedFilePath = await join(dir, fileName);
            await copyFile(filePath, copiedFilePath);
        }
        await readImportFiles();
    }

    async function selectFilesToImport() {
        if (workingDir) {
            const files = await open({
                directory: false,
                multiple: true,
            });
            await importNewFiles(files);
        }
    }

    async function saveProject() {
        const filePath = await save({
            filters: [
                {
                    name: "Zouna Project",
                    extensions: ["zp"],
                },
            ],
        });
        // console.log(filePath);
    }

    async function openGame() {
        gameFiles.clear();
        dpcFiles.clear();
        openObjects.clear();
        currentObject = null;

        const registered = await isRegistered("Control+S");
        if (!registered) {
            // await register("Control+S", saveProject);
            // await register("Control+Shift+S", saveProject);
        }

        toggleDropdown("project");

        const dir = await open({
            directory: true,
        });

        if (dir) {
            workingDir = dir;
            let files = await readDir(dir, { recursive: true });
            gameFiles = processEntries(files, sortByName);
            await readImportFiles();
        }
    }

    async function openDPC(event) {
        dpcFiles.clear();
        openObjects.clear();
        currentObject = null;

        const inPath = event.detail.file;
        currentDPC = inPath;
        const outPath = await join(workingDir, PROJECT_DIR, "DPCS");

        let outPathExists = await invoke("path_exists", { path: outPath });
        if (!outPathExists) await createDir(outPath, { recursive: true });

        let dpcOutPath = await invoke("get_dpc_out_path", {
            inPath: inPath,
            outPath: outPath,
        });
        let dpcOutPathExists = await exists(dpcOutPath);
        if (!dpcOutPathExists) {
            try {
                await invoke("read_dpc", {
                    inPath: inPath,
                    outPath: dpcOutPath,
                    game: game,
                });
            } catch (err) {
                console.log(`error ${err}`);
            }
        } else console.log("dir already exists"); // add a popup asking for overwrite
        let dpcOutFiles = await readDir(dpcOutPath, { recursive: true });
        dpcFiles = processEntries(dpcOutFiles, sortByExtension);
    }

    async function openObject(event) {
        const inPath = event.detail.file;
        if (!openObjects.has(inPath)) {
            const objectBasename = await basename(inPath);

            // objectPreview = null;
            // objectPreviewDesc = objectBasename;
            // objectProperties = null;
            // console.log(inPath);
            // currentDPC = inPath;
            const outPath = await join(
                workingDir,
                "ZOUNA_PROJECT",
                "OBJECTS",
                objectBasename
            );

            let outPathExists = await invoke("path_exists", { path: outPath });
            if (!outPathExists) await createDir(outPath, { recursive: true });

            // let dpcOutPath = await invoke("get_dpc_out_path", {
            //     inPath: inPath,
            //     outPath: outPath,
            // });
            // let dpcOutPathExists = await exists(dpcOutPath);
            try {
                await invoke("read_object", {
                    inPath: inPath,
                    outPath: outPath,
                    game: game,
                });
            } catch (err) {
                console.log(`error ${err}`);
            }
            const objectJsonPath = await join(outPath, "object.json");
            const objectJson = await readTextFile(objectJsonPath);
            if (objectJson.length > 1) {
                let properties = objectJson;
                let preview;
                let dataName;
                // objectProperties = JSON.parse(objectJson);
                // console.log(objectProperties);
                let objectOutFiles = await readDir(outPath, {
                    recursive: false,
                });
                for (const entry of objectOutFiles) {
                    switch (entry.name) {
                        case "data.dds":
                            const ddsName = await basename(entry.path, "dds");
                            const pngPath = await join(
                                outPath,
                                ddsName + "png"
                            );
                            const pngExists = await exists(pngPath);
                            if (!pngExists) {
                                try {
                                    await invoke("save_png", {
                                        ddsPath: entry.path,
                                    });
                                    preview = convertFileSrc(pngPath);
                                    // console.log(objectPreview);
                                } catch (err) {
                                    console.log(err);
                                }
                            } else {
                                preview = convertFileSrc(pngPath);
                            }
                            dataName = entry.name;
                            break;
                        case "data.wav":
                            preview = convertFileSrc(entry.path);
                            dataName = entry.name;
                            break;
                        case "data.obj":
                            preview = convertFileSrc(entry.path);
                            dataName = entry.name;
                            break;
                        case "data.txt":
                            preview = entry.path;
                            dataName = entry.name;
                            break;
                    }
                    // console.log(objectPreview);
                }
                openObjects.set(
                    inPath,
                    new Map([
                        ["name", objectBasename],
                        ["dataname", dataName],
                        ["properties", properties],
                        ["preview", preview],
                        ["replacement", null],
                    ])
                );
            } else {
                openObjects.set(
                    inPath,
                    new Map([
                        ["name", objectBasename],
                        ["dataname", null],
                        ["properties", null],
                        ["preview", null],
                        ["replacement", null],
                    ])
                );
                // objectPreviewDesc =
                // objectPreviewDesc + " - unable to read this file yet";
            }
            // dpcFiles = processEntries(dpcOutFiles);
        }
        currentObject = inPath;
        // console.log(openObjects);
    }

    async function openOtherFile(event) {
        const inPath = event.detail.file;
        const fileBasename = await basename(inPath);
        const ext = await extname(inPath);
        let convertedPath = event.detail.file;
        if (FORMATS.get("audio").includes(ext))
            convertedPath = await convertFileSrc(inPath);
        if (FORMATS.get("image").includes(ext))
            convertedPath = await convertFileSrc(inPath);
        if (FORMATS.get("model").includes(ext))
            convertedPath = await convertFileSrc(inPath);
        openObjects.set(
            inPath,
            new Map([
                ["name", fileBasename],
                ["dataname", null],
                ["properties", null],
                ["preview", convertedPath],
                ["replacement", null],
            ])
        );
        currentObject = inPath;
    }

    function replaceObjectData(event) {
        if (currentObject && objectProperties) {
            const inPath = event.detail.file;
            openObjects.get(currentObject).set("replacement", inPath);
            openObjects = openObjects; //svelte shenanigans
        }
    }

    function resetObjectData() {
        openObjects.get(currentObject).set("replacement", null);
        openObjects = openObjects; //svelte shenanigans
    }

    function removeTab(object) {
        openObjects.delete(object);
        openObjects = openObjects;
        if (currentObject == object)
            currentObject = openObjects.keys().next().value;
    }

    $: imagePromise = new Promise((resolve) => {
        if (currentObject && objectPreview) {
            const image = new Image();

            image.onload = () =>
                resolve({
                    width: image.width,
                    height: image.height,
                    padding: 32,
                    maxZoom: 64,
                    friction: 0,
                    render,
                });
            image.src = objectPreview;

            function render(ctx, t) {
                ctx.drawImage(image, 0, 0);
            }
        }
    });
</script>

<div class="window">
    <div class="menubar">
        <div class="menubar-buttons">
            <div>
                <button on:click={() => toggleDropdown("project")}
                    >project</button
                >
                <div class="dropdown" hidden={!dropdowns.get("project")}>
                    <div class="dropdown-menu">
                        <button on:click={openGame}
                            >New <span>(Ctrl+N)</span></button
                        >
                        <button>Open <span>(Ctrl+O)</span></button>
                        <button on:click={saveProject} disabled={isProjectOpen}
                            >Save <span>(Ctrl+S)</span></button
                        >
                        <button on:click={saveProject} disabled={isProjectOpen}
                            >Save As <span>(Ctrl+Shift+S)</span></button
                        >
                        <button
                            on:click={selectFilesToImport}
                            disabled={isProjectOpen}>Import Files</button
                        >
                    </div>
                </div>
            </div>
            <div>
                <button
                    on:click={() => toggleDropdown("export")}
                    disabled={isProjectOpen}>export</button
                >
                <div class="dropdown" hidden={!dropdowns.get("export")}>
                    <div class="dropdown-menu">
                        <button>export modified files</button>
                        <button>pack current dpc</button>
                    </div>
                </div>
            </div>
            <div>
                <button on:click={() => toggleDropdown("game")}>game</button>
                <div class="dropdown" hidden={!dropdowns.get("game")}>
                    <div class="dropdown-menu">
                        <label>
                            <input
                                type="radio"
                                bind:group={game}
                                name="game"
                                value={"walle"}
                            /> WALL-E
                        </label>
                        <label>
                            <input
                                type="radio"
                                bind:group={game}
                                name="game"
                                value={"fuel"}
                            /> Fuel
                        </label>
                        <label>
                            <input
                                type="radio"
                                bind:group={game}
                                name="game"
                                value={"other"}
                            /> Other (unsupported)
                        </label>
                    </div>
                </div>
            </div>
        </div>
        <div class="title">
            {workingDir.split(sep).slice(-1)}
            {#if currentDPC} - {currentDPC.replace(workingDir + sep, "")}{/if}
        </div>
    </div>
    <div class="editor">
        <div class="object-tabs" on:mousedown|preventDefault>
            {#if currentObject}
                {#each [...openObjects.keys()] as object}
                    <button
                        on:mousedown={(e) => {
                            // console.log(e);
                            if (e.button == 0) currentObject = object;
                            if (e.button == 1) removeTab(object);
                        }}
                        style="background-color: {currentObject == object
                            ? 'var(--border-bg)'
                            : 'var(--container-bg)'};"
                        >{openObjects.get(object).get("name")}
                        <button on:click={removeTab(object)}>✕</button></button
                    >
                {/each}
            {:else}
                <button>no objects</button>
            {/if}
        </div>
        <Container name="Game Structure" className="game">
            {#each [...gameFiles.keys()] as file}
                <File
                    name={file}
                    subfiles={gameFiles.get(file)}
                    path={workingDir + sep + file}
                    on:opendpc={openDPC}
                    on:openfile={openOtherFile}
                />
            {/each}
        </Container>
        <Container name="DPC Structure" className="dpc">
            {#each [...dpcFiles.keys()] as file}
                <File
                    name={file}
                    subfiles={dpcFiles.get(file)}
                    path={workingDir +
                        sep +
                        "ZOUNA_PROJECT" +
                        sep +
                        "DPCS" +
                        sep +
                        currentDPC.split(sep).slice(-1) +
                        ".d" +
                        sep +
                        file}
                    on:openobj={openObject}
                    on:openfile={openOtherFile}
                />
            {/each}
        </Container>
        <Container name="Imported Files" className="imported">
            {#each [...importFiles.keys()] as file}
                <File
                    name={file}
                    subfiles={importFiles.get(file)}
                    path={workingDir +
                        sep +
                        "ZOUNA_PROJECT" +
                        sep +
                        "IMPORTS" +
                        sep +
                        file}
                    imported={true}
                    on:openimport={replaceObjectData}
                    on:openfile={openOtherFile}
                />
            {/each}
        </Container>
        <Container name="Preview" className="preview">
            {#if currentObject}
                {#if objectPreview}
                    {#if FORMATS.get("image").includes(String(objectPreview
                                .split(".")
                                .slice(-1)))}
                        {#await imagePromise then options}
                            <canvas use:panzoom={options} />
                        {/await}
                        <img
                            src={objectPreview}
                            alt="preview"
                            style="width: 100%;"
                        />
                    {:else if FORMATS.get("audio").includes(String(objectPreview
                                .split(".")
                                .slice(-1)))}
                        <audio controls src={objectPreview} />
                    {:else if FORMATS.get("model").includes(String(objectPreview
                                .split(".")
                                .slice(-1)))}
                        <!-- <Canvas>
                            <T.PerspectiveCamera
                                makeDefault
                                fov={90}
                                position={[2, 2, 2]}
                                on:create={({ ref }) => {
                                    ref.lookAt(0, 0, 0);
                                }}
                            >
                                <OrbitControls enableZoom={true} />
                            </T.PerspectiveCamera>
                            <T.AmbientLight intensity={0.4} />
                            <T.DirectionalLight
                                position={[7, 7, -1]}
                                castShadow={true}
                                intensity={2}
                            />
                        </Canvas> -->
                        <div bind:this={sceneContainer} class="renderer" />
                    {:else}
                        {#await readTextFile(objectPreview) then text}
                            <div class="text-preview">
                                <span class="monospace-text">{text}</span>
                            </div>
                        {/await}
                    {/if}
                {:else if objectProperties}
                    <span>object preview unavailable</span>
                {:else}
                    <span>object not supported yet</span>
                {/if}
            {/if}
        </Container>
        <Container name="Properties" className="properties">
            {#if currentObject}
                <span
                    >Object Data: {#if objectReplacement}<span
                            style="color: aquamarine"
                            >{objectReplacement.split(sep).slice(-1)}</span
                        >
                        <button class="reset-button" on:click={resetObjectData}
                            >Reset</button
                        >{:else if objectPreview}{openObjects
                            .get(currentObject)
                            .get("dataname")}{:else}default{/if}</span
                >
                <hr />
                <div class="monospace-text">
                    {objectProperties}
                </div>
            {/if}
            <!-- {#if objectProperties}
                {objectProperties}
            {/if} -->
            <!-- {#each [...Object.entries(objectProperties)] as [property, val]}
                <Property propertyName={property} subproperties={val} />
            {/each} -->
        </Container>
    </div>
</div>

<svelte:body on:contextmenu|preventDefault />

<svelte:window on:resize={resizeScene} />

<style>
    @import url("https://fonts.googleapis.com/css2?family=DM+Sans&display=swap");
    @import url("https://fonts.googleapis.com/css2?family=Ubuntu+Mono&display=swap");

    * {
        font-family: "DM Sans", sans-serif;
    }

    *::-webkit-scrollbar {
        background-color: var(--dark-bg);
        width: 10px;
    }
    *::-webkit-scrollbar-thumb {
        background-color: hsl(0, 16%, 30%);
    }
    *::-webkit-scrollbar-thumb:hover {
        background-color: hsl(0, 17%, 33%);
    }

    :global(html) {
        height: 100%;
        user-select: none;
        color-scheme: dark;
        overflow: hidden;
        --dark-bg: hsl(0, 5%, 7%);
        --window-bg: hsl(0, 7%, 9%);
        --container-bg: hsl(0, 10%, 15%);
        --border-bg: hsl(0, 15%, 20%);
        --light-bg: hsl(0, 18%, 30%);
        --lighter-bg: hsl(0, 20%, 37%);
    }

    :global(body) {
        margin: 0;
        height: 100%;
        max-width: 100vw;
        max-height: 100vh;
    }

    .window {
        display: flex;
        background-color: var(--window-bg);
        height: 100%;
        flex-direction: column;
        /* grid-template-rows: min-content 100%; */
        overflow: hidden;
    }

    .menubar {
        display: flex;
        justify-content: space-between;
        max-width: 100%;
        padding-inline: 4px;
        background-color: var(--container-bg);
        font-size: 15px;
        color: antiquewhite;
        /* border-bottom: 2px solid cornflowerblue; */
    }

    .menubar button {
        font-size: 15px;
    }

    .menubar-buttons {
        display: flex;
    }

    .dropdown {
        position: absolute;
        /* transform: translateY(24px); */
    }

    .dropdown-menu {
        display: flex;
        border: 4px solid var(--border-bg);
        flex-direction: column;
        min-width: 10ch;
        box-shadow: 4px 4px hsla(0, 5%, 7%, 80%);
    }

    .dropdown button {
        display: flex;
        justify-content: space-between;
        text-align: left;
    }

    .dropdown button span {
        margin-left: 1em;
    }

    .editor {
        display: grid;
        margin: 4px;
        gap: 4px;
        height: 100%;
        max-height: calc(100% - 30px);
        grid-template-columns: 1fr 1.7fr 1fr;
        grid-template-rows: 2em 1fr 1fr;
        grid-template-areas:
            "game object-tabs object-tabs"
            "game preview properties"
            "imported preview dpc";
    }

    .object-tabs {
        grid-area: object-tabs;
        display: flex;
        flex-direction: row;
        flex-wrap: nowrap;
        overflow-x: scroll;
    }

    .object-tabs::-webkit-scrollbar {
        height: 6px;
    }

    .object-tabs button {
        white-space: nowrap;
    }

    button,
    label {
        border: 0;
        background-color: var(--container-bg);
        color: antiquewhite;
    }

    button:hover,
    label:hover {
        background-color: var(--border-bg);
    }

    button:disabled {
        color: gray;
    }

    canvas {
        box-sizing: border-box;
        width: 100%;
        height: calc(100% - 8px);
        user-select: none;
        touch-action: none;
        background-color: transparent;
        overscroll-behavior: none;
        -webkit-user-select: none; /* disable selection/Copy of UIWebView */
        -webkit-touch-callout: none; /* disable the IOS popup when long-press on a link */
    }

    hr {
        width: 100%;
    }

    .monospace-text {
        font-family: "Ubuntu Mono", monospace;
    }

    .text-preview {
        margin: -4px;
        padding: 4px;
        height: calc(100% - 4px);
        white-space: pre;
        overflow: auto;
        user-select: text;
        /* width: 100%; */
    }

    .renderer {
        width: 100%;
        height: 100%;
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
