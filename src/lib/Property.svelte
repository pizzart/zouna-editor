<script>
    // @ts-nocheck

    export let propertyName;
    export let value;
    export let subproperties;
</script>

<div>
    {propertyName}:
    {#if subproperties}
        <div class="property-inner">
            {#each [...Object.entries(subproperties)] as [property, val]}
                {#if val instanceof Object && !(val instanceof Array)}
                    <svelte:self
                        propertyName={property}
                        subproperties={val}
                        value=""
                    />
                {:else}
                    <svelte:self
                        propertyName={property}
                        value={val}
                        subproperties=""
                    />
                {/if}
            {/each}
        </div>
    {:else if value instanceof Array}
        {#if value.length() < 128}
            {value}
        {:else}
            ...
        {/if}
    {:else}
        {value}
    {/if}
</div>

<style>
    .property-inner {
        margin-left: 1ch;
    }
</style>
