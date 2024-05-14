<script lang="ts">
    export let property_name: string;
    export let property_value: string;
    export let message: string;
</script>

<div id="tip">
    <p>{property_name}</p>
    <p>{property_value}</p>
    <p>{message}</p>
</div>