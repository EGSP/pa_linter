<script lang="ts">
	import IconExclamationTriangle from '$lib/icons/IconExclamationTriangle.svelte';
	import { type AnalysisResult, type RepositoryTree } from '$lib/types';
	import { Accordion, AccordionItem, TreeView } from 'carbon-components-svelte';
	import { invoke } from '@tauri-apps/api';
	import IAnalysisResult from './analyze/IAnalysisResult.svelte';
	import IProjectArenaTree from './structure/IProjectArenaTree.svelte';
	import { onMount } from 'svelte';
	import Frame from '$lib/components/Frame.svelte';
	import CarbonRun from '$lib/icons/CarbonRun.svelte';

	import { Button } from 'carbon-components-svelte';
	import Label from '$lib/components/Label.svelte';
	import type { TreeNode } from 'carbon-components-svelte/src/TreeView/TreeView.svelte';

	import { Pane, Splitpanes } from 'svelte-splitpanes';

	let analysis_results = new Array<AnalysisResult>();
	async function analyze() {
		return;
		analysis_results = await invoke<AnalysisResult[]>('analyze_folder', {
			folderPath: 'c:/Workroot/softdev/pa_linter_test'
		});

		console.log(analysis_results);
	}

	let project_tree: TreeNode[] = [];
	async function get_project_tree() {

		let trees = await invoke<RepositoryTree[]>('c_get_project_trees')
		console.log(trees);
		// let arena_tree_corrupted: ArenaTree;
		// arena_tree_corrupted = await invoke<ArenaTree>('get_project_folder_arena_tree', {
		// 	folderPath: 'c:/Workroot/softdev/pa_linter_test_tree/Consultant-Balance-main'
		// });
		// let nodes_array = arena_tree_corrupted['nodes_map'] as unknown as Array<Node>;

		// project_tree = new Array<TreeNode>();

		// let nodes_map = new Map<string, Node>();
		// for (let i in nodes_array) {
		// 	nodes_map.set(nodes_array[i].id.toString(), nodes_array[i]);
		// }

		// let root_node = nodes_map.get('0');

		// if (root_node) {
		// 	project_tree.push({
		// 		id: root_node.id.toString(),
		// 		text: root_node.value,
		// 		children: get_children(root_node.id.toString())
		// 	});
		// }

		// function get_children(node_id: string) {
		// 	let arena_node = nodes_map.get(node_id);
		// 	let children = new Array<TreeNode>();
		// 	if (arena_node) {
		// 		for (let child_id of arena_node.children) {
		// 			let child_node = nodes_map.get(child_id.toString());
		// 			if (child_node) {
		// 				children.push({
		// 					id: child_node.id.toString(),
		// 					text: child_node.value,
		// 					children: get_children(child_node.id.toString())
		// 				});
		// 			}
		// 		}
		// 	}

		// 	return children;
		// }
	}

	onMount(async () => {
		await get_project_tree();
		// await analyze_tree();
	});
</script>

<Frame>
	<Splitpanes >
		<Pane>
			<Frame direction={'column'}>
				<div class="action-bar">
					<Button
						on:click={analyze}
						kind="primary"
						size="small"
						icon={CarbonRun}
						iconDescription="Analyze project folder"
						tooltipPosition="right"
						tooltipAlignment="end"
					/>
				</div>

				<div class="project-tree" id="project-tree">
					{#if project_tree}
						<TreeView
							labelText="Project Tree"
							children={project_tree}
							style="overflow:unset"
						/>
					{/if}
				</div>
			</Frame>
		</Pane>
		<Pane>
			<Frame direction={'column'}>
				<Accordion align="start" size="sm">
					{#if analysis_results.length > 0}
						{#each analysis_results as result, i}
							<AccordionItem>
								<svelte:fragment slot="title">
									<div class="flex">
										<IconExclamationTriangle/>
										<!-- {result.file_path} -->
										<Label text={result.file_path} />
									</div>
								</svelte:fragment>
								<IAnalysisResult tips={result.tips} />
							</AccordionItem>
						{/each}
					{:else}
						<p>No results</p>
					{/if}
				</Accordion>
			</Frame>
		</Pane>
	</Splitpanes>
</Frame>

<style>
	.project-tree {
		display: flex;
		flex-direction: column;
		text-wrap: nowrap;

		width: 100%;
		height: 100%;
	}

	.action-bar {
		display: flex;
		flex: none;
		flex-flow: row;
		width: 100%;
	}
</style>
