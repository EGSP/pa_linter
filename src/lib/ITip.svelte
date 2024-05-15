<script lang="ts">
	export let property_name: string;
	export let property_value: string;
	export let message: string;
</script>

<div class="variant-soft-surface" id="tip">
	<span class="badge variant-soft-surface">property: {property_name}</span>
	<span class="badge variant-soft-surface">value: {property_value}</span>
	<p class="variant-ghost-warning pl-1">{message}</p>
</div>
