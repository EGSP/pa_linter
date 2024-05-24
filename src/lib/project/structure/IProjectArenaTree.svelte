<script lang="ts">
	import { TreeView } from '@skeletonlabs/skeleton';
	import { Node, type ArenaTree } from "$lib/types";
	import IProjectArenaTreeItem from '$lib/project/structure/IProjectArenaTreeItem.svelte';

	export let project_arena_tree: ArenaTree;
    export let project_arena_tree_length: number;
</script>

<div >
    <div>Arena Length: {project_arena_tree_length}</div>
	<TreeView >
		{#if project_arena_tree && project_arena_tree.nodes_map.size > 0}
                <IProjectArenaTreeItem project_arena_tree={project_arena_tree} node={project_arena_tree.nodes_map.get("0") || new Node()} />
		{:else}
			<p>No nodes</p>
		{/if}
	</TreeView>
</div>

