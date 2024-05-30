<script lang="ts">
	export let direction: string = 'row';
</script>

<div class="flow-frame" style="flex-direction:{direction};">
	<slot />
</div>

<style>
	.flow-frame {
		display: flex;
	}
</style>
