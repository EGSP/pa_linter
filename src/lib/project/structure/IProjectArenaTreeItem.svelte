<script lang="ts">
	import { TreeViewItem } from '@skeletonlabs/skeleton';
	import type { ArenaTree, Node } from "$lib/types";
    import IProjectArenaTreeItem from './IProjectArenaTreeItem.svelte';

	export let project_arena_tree: ArenaTree;
	export let node: Node;

    let children: Node[] = [];

    // get children nodes if they not undefinded
    if (node.children.length > 0) {
        for (let child_node_id of node.children) {
            let child_node = project_arena_tree.nodes_map.get(child_node_id.toString());
            if (child_node) {
                console.log(node.value + " Child push arena_item: " + child_node_id);
                children.push(child_node);
            }else {
                console.log("Node not found arena_item: " + child_node_id);
            }
        }
    }
    
</script>

<TreeViewItem >
	<span>{node.value}</span>
	<span>{node.children.length}</span>

	<svelte:fragment slot="children">
		{#if children.length > 0}
			{#each children as child_node}
				<IProjectArenaTreeItem
					{project_arena_tree}
					node={child_node}
				/>
			{/each}
		{/if}
	</svelte:fragment>
</TreeViewItem>

