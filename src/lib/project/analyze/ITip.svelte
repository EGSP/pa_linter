<script lang="ts">
	import FlowFrame from "$lib/components/FlowFrame.svelte";
import Frame from "$lib/components/Frame.svelte";
import Label from "$lib/components/Label.svelte";

	export let property_name: string;
	export let property_value: string;
	export let message: string;
</script>

<FlowFrame direction={'column'}>
	<Label text="property:{property_name}" />
	<Label text="value:{property_value}" />
	<Label text="message:{message}" />
</FlowFrame>
