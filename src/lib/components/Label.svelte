<script lang="ts">
    export let text: string;
    export let wrap: boolean = false;
</script>

<span class="label" style="text-wrap:{wrap?'unset':'nowrap'};">{text}</span>

<style>
    .label {
        overflow-wrap: anywhere;
    }
</style>